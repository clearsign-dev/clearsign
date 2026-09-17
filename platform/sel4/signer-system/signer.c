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
    const uint8_t *req = (const uint8_t *)request_vaddr;
    uint8_t *rev = (uint8_t *)review_vaddr;

    uint32_t len = ((uint32_t)req[0] << 24) | ((uint32_t)req[1] << 16) | ((uint32_t)req[2] << 8) | req[3];
    if (len > REGION_SIZE - REQ_HEADER) {
        microkit_dbg_puts("SIGNER|refused: request length exceeds region\n");
        rev[0] = 'E';
        microkit_notify(CH_WALLET_UI);
        return;
    }

    uint8_t severity = 0xff;
    long n = clearsign_review(req + REQ_HEADER, len, rev + REV_HEADER, REGION_SIZE - REV_HEADER, &severity);
    if (n < 0) {
        microkit_dbg_puts("SIGNER|refused: request could not be decoded\n");
        rev[0] = 'E';
        microkit_notify(CH_WALLET_UI);
        return;
    }

    rev[1] = severity;
    for (int i = 0; i < 8; i++) {
        rev[8 + i] = (uint8_t)((uint64_t)n >> (56 - 8 * i));
    }
    rev[0] = 'R';

    microkit_dbg_puts("SIGNER|review computed inside the signer compartment:\n");
    microkit_dbg_puts((const char *)(rev + REV_HEADER));
    microkit_dbg_puts("SIGNER|highest severity: ");
    microkit_dbg_puts(severity_name(severity));
    microkit_dbg_puts("\n");
    microkit_notify(CH_WALLET_UI);
}
