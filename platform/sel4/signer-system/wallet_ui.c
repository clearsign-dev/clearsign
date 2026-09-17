/*
 * Wallet UI protection domain. UNTRUSTED by design: in the threat model this is
 * the compromised interface (T1/T2). It may write requests, may read reviews,
 * and has no capability to modify a review. The last thing it does is try.
 */
#include <stdint.h>
#include <stddef.h>
#include <microkit.h>
#include "request_bybit_pattern.h"

#define CH_SIGNER 1
#define REQ_HEADER 4

uintptr_t request_vaddr;
uintptr_t review_vaddr;

void init(void)
{
    uint8_t *req = (uint8_t *)request_vaddr;
    uint32_t len = sizeof(REQUEST_BYBIT_PATTERN);
    req[0] = (uint8_t)(len >> 24);
    req[1] = (uint8_t)(len >> 16);
    req[2] = (uint8_t)(len >> 8);
    req[3] = (uint8_t)len;
    for (uint32_t i = 0; i < len; i++) {
        req[REQ_HEADER + i] = REQUEST_BYBIT_PATTERN[i];
    }
    microkit_dbg_puts("WALLET_UI|submitted a Safe transaction for review (the UI calls it a token transfer)\n");
    microkit_notify(CH_SIGNER);
}

void notified(microkit_channel ch)
{
    if (ch != CH_SIGNER) {
        return;
    }
    const uint8_t *rev = (const uint8_t *)review_vaddr;
    if (rev[0] != 'R') {
        microkit_dbg_puts("WALLET_UI|signer refused the request\n");
        return;
    }
    microkit_dbg_puts(rev[1] == 4 ? "WALLET_UI|signer verdict received: CRITICAL\n"
                                  : "WALLET_UI|signer verdict received: not critical\n");

    microkit_dbg_puts("WALLET_UI|attempting to overwrite the signer's verdict with 'no risk'...\n");
    ((uint8_t *)review_vaddr)[1] = 0; /* review region is mapped read-only here: seL4 must fault */
    microkit_dbg_puts("WALLET_UI|ISOLATION FAILURE: write succeeded\n");
}
