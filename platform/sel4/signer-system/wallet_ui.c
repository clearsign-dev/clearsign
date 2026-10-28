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
    microkit_notify(CH_SIGNER);
}

void notified(microkit_channel ch)
{
    if (ch != CH_SIGNER) {
        return;
    }
    __atomic_thread_fence(__ATOMIC_ACQUIRE);
    const uint8_t *rev = (const uint8_t *)review_vaddr;
    if (rev[0] != 'R') {
        return;
    }
    ((uint8_t *)review_vaddr)[1] = 0; /* review region is mapped read-only here: seL4 must fault */
    /* Reaching this notification means the forbidden write was not blocked.
     * The trusted signer, not this untrusted domain, reports the failure. */
    __atomic_thread_fence(__ATOMIC_RELEASE);
    microkit_notify(CH_SIGNER);
}
