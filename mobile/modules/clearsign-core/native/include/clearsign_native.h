#ifndef CLEARSIGN_NATIVE_H
#define CLEARSIGN_NATIVE_H

#include <stddef.h>
#include <stdint.h>

uint32_t clearsign_api_version(void);
size_t clearsign_max_record_bytes(void);

intptr_t clearsign_review_safe_json(
    const uint8_t *input,
    size_t input_length,
    uint64_t chain_id,
    uint32_t version,
    uint8_t *output,
    size_t output_capacity
);

#endif
