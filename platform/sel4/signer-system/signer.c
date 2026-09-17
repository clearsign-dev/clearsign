/*
 * Signer protection domain.
 *
 * Capabilities (see signer.system): read-only view of the request region,
 * read-write view of the review region, one channel to the wallet UI.
 * No devices, no network, nothing else. Decoding runs in Rust (clearsign-ffi).
 */
#include <stdint.h>
#include <stddef.h>
#include <microkit.h>

#define CH_WALLET_UI 1
#define REGION_SIZE 0x10000
#define REQ_HEADER 4   /* big-endian u32 length */
#define REV_HEADER 16  /* [0] magic 'R', [1] severity, [8..16] text length */

uintptr_t request_vaddr;
uintptr_t review_vaddr;

extern long clearsign_review(const uint8_t *request, size_t request_len,
                             uint8_t *out, size_t out_cap, uint8_t *severity);

static const char *severity_name(uint8_t s)
{
    switch (s) {
    case 0: return "NONE";
    case 1: return "INFO";
    case 2: return "WARNING";
    case 3: return "BLIND";
    case 4: return "CRITICAL";
    default: return "?";
    }
}

void init(void)
{
    microkit_dbg_puts("SIGNER|ready. Request region is mapped read-only; no devices or network are mapped.\n");
}

void notified(microkit_channel ch)
{
    if (ch != CH_WALLET_UI) {
        return;
    }
    /* Both regions are written by another protection domain, so every access
     * must actually happen rather than be cached or elided by the compiler. */
    volatile const uint8_t *req = (volatile const uint8_t *)request_vaddr;
    volatile uint8_t *rev = (volatile uint8_t *)review_vaddr;

    /* Everything the wallet UI wrote before notifying must be visible first. */
    __atomic_thread_fence(__ATOMIC_ACQUIRE);

    uint32_t len = ((uint32_t)req[0] << 24) | ((uint32_t)req[1] << 16) | ((uint32_t)req[2] << 8) | req[3];
    if (len > REGION_SIZE - REQ_HEADER) {
        microkit_dbg_puts("SIGNER|refused: request length exceeds region\n");
        __atomic_thread_fence(__ATOMIC_RELEASE);
        rev[0] = 'E';
        microkit_notify(CH_WALLET_UI);
        return;
    }

    /* Copy the request into private memory before decoding. Reviewing bytes in
     * place would let the untrusted UI change them between checks, so the review
     * could describe something other than what was decoded (a TOCTOU attack). */
    static uint8_t private_copy[REGION_SIZE];
    for (uint32_t i = 0; i < len; i++) {
        private_copy[i] = req[REQ_HEADER + i];
    }

    uint8_t severity = 0xff;
    long n = clearsign_review(private_copy, len, (uint8_t *)(rev + REV_HEADER), REGION_SIZE - REV_HEADER, &severity);
    if (n < 0) {
        microkit_dbg_puts("SIGNER|refused: request could not be decoded\n");
        __atomic_thread_fence(__ATOMIC_RELEASE);
        rev[0] = 'E';
        microkit_notify(CH_WALLET_UI);
        return;
    }

    rev[1] = severity;
    for (int i = 0; i < 8; i++) {
        rev[8 + i] = (uint8_t)((uint64_t)n >> (56 - 8 * i));
    }
    /* Publish last, after everything it refers to is visible. */
    __atomic_thread_fence(__ATOMIC_RELEASE);
    rev[0] = 'R';

    microkit_dbg_puts("SIGNER|review computed inside the signer compartment:\n");
    microkit_dbg_puts((const char *)(rev + REV_HEADER));
    microkit_dbg_puts("SIGNER|highest severity: ");
    microkit_dbg_puts(severity_name(severity));
    microkit_dbg_puts("\n");
    microkit_notify(CH_WALLET_UI);
}
