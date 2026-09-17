/*
 * signer-request: runs inside the Linux guest. No libc; raw aarch64 syscalls only,
 * so it needs nothing from the guest's root filesystem.
 *
 *   signer-request          submit the Bybit-pattern Safe transaction, print the review
 *   signer-request plan     submit the plan a prompt-injected assistant would propose
 *   signer-request tamper   try to overwrite the signer's review (must be stopped by seL4)
 *
 * The "plan" case is the one that generalises: Linux here stands for the whole
 * untrusted world, including whatever AI assistant proposed the plan. It writes
 * the proposal into shared memory; the compartment on the other side decides
 * what that proposal actually does.
 */
#include "request_bybit_pattern.h"
#include "request_injected_plan.h"

typedef unsigned long u64;
typedef long i64;

#define REQUEST_GPA 0x60000000UL
#define REVIEW_GPA 0x60100000UL
#define DOORBELL_GPA 0x60200000UL
#define REGION 0x10000UL
#define PAGE 0x1000UL

/* Raw syscall: arguments in x0-x5, syscall number passed as the 7th argument (x6)
 * and moved into x8 by a real assembly function, so the ABI is explicit. */
extern i64 raw_syscall(i64 a, i64 b, i64 c, i64 d, i64 e, i64 f, i64 nr);
__asm__(".global raw_syscall\nraw_syscall:\n  mov x8, x6\n  svc #0\n  ret\n");
static i64 sys(i64 n, i64 a, i64 b, i64 c, i64 d, i64 e, i64 f) { return raw_syscall(a, b, c, d, e, f, n); }
#define SYS_openat 56
#define SYS_write 64
#define SYS_exit 93
#define SYS_nanosleep 101
#define SYS_mmap 222
#define AT_FDCWD -100
#define O_RDWR 2
#define O_SYNC 04010000
#define PROT_READ 1
#define PROT_WRITE 2
#define MAP_SHARED 1

static u64 strlen_(const char *s) { u64 n = 0; while (s[n]) n++; return n; }
static void out(const char *s) { sys(SYS_write, 1, (i64)s, (i64)strlen_(s), 0, 0, 0); }
static void die(const char *s) { out(s); sys(SYS_exit, 1, 0, 0, 0, 0, 0); for (;;) {} }

static void *map(i64 fd, u64 gpa, u64 size, int prot)
{
    i64 p = sys(SYS_mmap, 0, (i64)size, prot, MAP_SHARED, fd, (i64)gpa);
    if (p < 0 && p > -4096) die("signer-request: mmap of /dev/mem failed\n");
    return (void *)p;
}

void cmain(int argc, char **argv)
{
    i64 fd = sys(SYS_openat, AT_FDCWD, (i64)"/dev/mem", O_RDWR | O_SYNC, 0, 0, 0);
    if (fd < 0) die("signer-request: cannot open /dev/mem (run as root)\n");

    if (argc > 1 && argv[1][0] == 't') {
        /* Map the review region writable at the Linux level. Linux permits this;
         * the question is whether seL4's stage-2 mapping (read-only) does. */
        volatile unsigned char *rev = map(fd, REVIEW_GPA, REGION, PROT_READ | PROT_WRITE);
        out("GUEST|Linux kernel granted a writable mapping of the review region. Writing 'no risk' now...\n");
        rev[1] = 0;
        out("GUEST|ISOLATION FAILURE: the review was modified\n");
        sys(SYS_exit, 2, 0, 0, 0, 0, 0);
    }

    /* Which proposal this run submits. Both cross the same boundary the same way. */
    int is_plan = (argc > 1 && argv[1][0] == 'p');
    const unsigned char *payload = is_plan ? REQUEST_INJECTED_PLAN : REQUEST_BYBIT_PATTERN;
    u64 len = is_plan ? sizeof(REQUEST_INJECTED_PLAN) : sizeof(REQUEST_BYBIT_PATTERN);

    out("GUEST|stage: mapping shared regions\n");
    volatile unsigned char *req = map(fd, REQUEST_GPA, REGION, PROT_READ | PROT_WRITE);
    volatile const unsigned char *rev = map(fd, REVIEW_GPA, REGION, PROT_READ);
    volatile unsigned char *bell = map(fd, DOORBELL_GPA, PAGE, PROT_READ | PROT_WRITE);
    out("GUEST|stage: writing request\n");

    unsigned char seq[4] = { 0x5e, 0x9b, 0x00, 0x01 };
    req[0] = (unsigned char)(len >> 24); req[1] = (unsigned char)(len >> 16);
    req[2] = (unsigned char)(len >> 8);  req[3] = (unsigned char)len;
    for (int i = 0; i < 4; i++) req[4 + i] = seq[i];
    for (u64 i = 0; i < len; i++) req[8 + i] = payload[i];

    out(is_plan ? "GUEST|submitting an assistant's action plan to the signer compartment\n"
                : "GUEST|submitting a Safe transaction to the signer compartment\n");
    bell[0] = 1; /* traps to the VMM, which notifies the signer */

    struct { i64 s, ns; } ts = { 0, 10000000 };
    for (int tries = 0; tries < 1000; tries++) {
        if (rev[0] && rev[2] == seq[0] && rev[3] == seq[1] && rev[4] == seq[2] && rev[5] == seq[3]) break;
        sys(SYS_nanosleep, (i64)&ts, 0, 0, 0, 0, 0);
    }
    if (rev[0] != 'R') die("GUEST|signer refused or did not answer\n");

    u64 n = 0;
    for (int i = 0; i < 8; i++) n = (n << 8) | rev[8 + i];
    out("GUEST|review received from the signer compartment:\n");
    /* Copy out byte by byte: never hand shared memory straight to a syscall. */
    static char text[0x10000];
    if (n > sizeof(text)) n = sizeof(text);
    for (u64 i = 0; i < n; i++) text[i] = (char)rev[16 + i];
    sys(SYS_write, 1, (i64)text, (i64)n, 0, 0, 0);
    out(rev[1] == 4 ? "GUEST|verdict: CRITICAL\n" : "GUEST|verdict: not critical\n");
    sys(SYS_exit, rev[1] == 4 ? 3 : 0, 0, 0, 0, 0, 0);
}

__asm__(".global _start\n_start:\n  ldr x0, [sp]\n  add x1, sp, #8\n  bl cmain\n  mov x8, #93\n  svc #0\n");
