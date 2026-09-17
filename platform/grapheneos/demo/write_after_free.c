/* A classic memory-safety bug: writing to memory after freeing it.
 *
 * With the standard glibc allocator the corruption goes unnoticed and the program
 * carries on. GrapheneOS's hardened_malloc zeroes freed memory and checks it is
 * still zero when the slot is reused, so it stops the program instead.
 * The reuse loop follows hardened_malloc's own test, test/write_after_free_small.c. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((optnone, noinline)) int main(void)
{
    char *secret = malloc(128);
    if (!secret) {
        return 1;
    }
    strcpy(secret, "session key material");
    free(secret);

    secret[65] = 'A';                       /* BUG: write after free */

    for (size_t i = 0; i < 100000; i++) {   /* churn the allocator until the slot is reused */
        free(malloc(128));
    }
    printf("DEMO|write-after-free went UNDETECTED\n");
    return 0;
}
