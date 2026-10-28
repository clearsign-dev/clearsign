/*
 * Trusted display protection domain.
 *
 * This is the only domain with the physical UART capability. The signer writes
 * the review into the read-only mapping below and notifies this domain. The
 * untrusted wallet can neither map the UART nor impersonate DISPLAY output.
 */
#include <stddef.h>
#include <stdint.h>
#include <microkit.h>

#define CH_SIGNER 1
#define REV_HEADER 16
#define REGION_SIZE 0x10000
#define UART_DR 0x00
#define UART_FR 0x18
#define UART_FR_TXFF (1u << 5)

uintptr_t review_vaddr;
uintptr_t serial_vaddr;

static void uart_putc(char c)
{
    volatile uint32_t *fr = (volatile uint32_t *)(serial_vaddr + UART_FR);
    volatile uint32_t *dr = (volatile uint32_t *)(serial_vaddr + UART_DR);
    while ((*fr & UART_FR_TXFF) != 0) {
    }
    *dr = (uint32_t)(unsigned char)c;
}

static void uart_puts(const char *s)
{
    while (*s != 0) {
        if (*s == '\n') {
            uart_putc('\r');
        }
        uart_putc(*s);
        s++;
    }
}

static const char *severity_name(uint8_t severity)
{
    switch (severity) {
    case 0: return "NONE";
    case 1: return "INFO";
    case 2: return "WARNING";
    case 3: return "BLIND";
    case 4: return "CRITICAL";
    default: return "INVALID";
    }
}

static void print_review(const volatile uint8_t *review)
{
    if (review[0] == 'F') {
        uart_puts("DISPLAY|ISOLATION FAILURE: wallet continued after the forbidden write\n");
        return;
    }
    if (review[0] != 'R') {
        uart_puts("DISPLAY|signer refused the request\n");
        return;
    }
    uint64_t len = 0;
    for (int i = 0; i < 8; i++) {
        len = (len << 8) | review[8 + i];
    }
    if (len > REGION_SIZE - REV_HEADER) {
        uart_puts("DISPLAY|invalid review length\n");
        return;
    }
    uart_puts("DISPLAY|review computed by signer:\n");
    uart_puts("DISPLAY|REVIEW|");
    for (uint64_t i = 0; i < len; i++) {
        char c = (char)review[REV_HEADER + i];
        if (c == '\n') {
            uart_puts("\nDISPLAY|REVIEW|");
        } else if (c >= 0x20 && c < 0x7f) {
            uart_putc(c);
        } else {
            uart_putc('.');
        }
    }
    uart_puts("\nDISPLAY|highest severity: ");
    uart_puts(severity_name(review[1]));
    uart_puts("\n");
    uart_puts("DISPLAY|trusted signer started the forbidden-write probe\n");
}

void init(void)
{
    uart_puts("DISPLAY|trusted UART ready\n");
}

void notified(microkit_channel ch)
{
    if (ch == CH_SIGNER) {
        __atomic_thread_fence(__ATOMIC_ACQUIRE);
        print_review((volatile const uint8_t *)review_vaddr);
    }
}
