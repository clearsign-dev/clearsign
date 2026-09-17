/*
 * Signer protection domain, serving a Linux guest.
 *
 * Capabilities (integrated.system): read-only view of the request region,
 * read-write view of the review region, one channel to the VMM. No devices,
 * no network, no access to guest RAM. Decoding runs in Rust (clearsign-ffi).
 *
 * Request region: [0..4] big-endian length, [4..8] sequence number, [8..] request frame.
 * Review region:  [0] 'R' or 'E', [1] severity, [2..6] echoed sequence, [8..16] text length, [16..] text.
 */
#include <stdint.h>
#include <stddef.h>
#include <microkit.h>

#define CH_VMM 1
#define REGION_SIZE 0x10000
#define REQ_HEADER 8
#define REV_HEADER 16

uintptr_t request_vaddr;
uintptr_t review_vaddr;

extern long clearsign_review(const uint8_t *request, size_t request_len,
                             uint8_t *out, size_t out_cap, uint8_t *severity);

void init(void)
{
    microkit_dbg_puts("SIGNER|ready to serve the Linux compartment. No guest RAM, devices or network are mapped.\n");
}

void notified(microkit_channel ch)
{
    if (ch != CH_VMM) {
        return;
    }
    const uint8_t *req = (const uint8_t *)request_vaddr;
    uint8_t *rev = (uint8_t *)review_vaddr;

    /* Snapshot the header once: the guest can keep writing to the shared region. */
    uint32_t len = ((uint32_t)req[0] << 24) | ((uint32_t)req[1] << 16) | ((uint32_t)req[2] << 8) | req[3];
    uint8_t seq[4] = { req[4], req[5], req[6], req[7] };

    rev[0] = 0; /* invalidate any previous review before doing anything else */
    for (int i = 0; i < 4; i++) {
        rev[2 + i] = seq[i];
    }
    if (len > REGION_SIZE - REQ_HEADER) {
        microkit_dbg_puts("SIGNER|refused: request length exceeds region\n");
        rev[0] = 'E';
        microkit_notify(CH_VMM);
        return;
    }

    /* Copy the request into private memory before decoding. Reviewing bytes in
     * place would let a racing guest change them between checks (a TOCTOU attack). */
    static uint8_t private_copy[REGION_SIZE];
    for (uint32_t i = 0; i < len; i++) {
        private_copy[i] = req[REQ_HEADER + i];
    }

    uint8_t severity = 0xff;
    long n = clearsign_review(private_copy, len, rev + REV_HEADER, REGION_SIZE - REV_HEADER, &severity);
    if (n < 0) {
        microkit_dbg_puts("SIGNER|refused: request could not be decoded\n");
        rev[0] = 'E';
        microkit_notify(CH_VMM);
        return;
    }
    rev[1] = severity;
    for (int i = 0; i < 8; i++) {
        rev[8 + i] = (uint8_t)((uint64_t)n >> (56 - 8 * i));
    }
    rev[0] = 'R';
    microkit_dbg_puts("SIGNER|review written for the Linux compartment (severity code ");
    char code[2] = { (char)('0' + (severity & 7)), 0 };
    microkit_dbg_puts(code);
    microkit_dbg_puts(")\n");
    microkit_notify(CH_VMM);
}
