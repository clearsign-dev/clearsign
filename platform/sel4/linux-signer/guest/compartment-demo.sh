#!/bin/sh
# What the Linux compartment does, unattended.
#
# There is no shell on this console and no way to type into it: the guest has no
# serial device at all. Its output is relayed by the VMM behind a prefix it
# cannot forge, and there is no receive path, so nothing outside can drive it.
# That is the point — the operator reads a console the untrusted side can only
# write to, one line at a time, and never types into the compartment.
#
# Each step prints a marker the test harness looks for.
echo "DEMO|starting"

echo "HM_MAPS=$(grep -c libhardened_malloc /proc/self/maps)"
echo HM_MAPS_DONE

write_after_free; echo "WAF_EXIT=$?"
write_after_free_static; echo "WAFS_EXIT=$?"

signer-request; echo "REQUEST_EXIT=$?"
signer-request plan; echo "PLAN_EXIT=$?"

# Last, because a blocked write stops this vCPU for good.
signer-request tamper; echo "TAMPER_EXIT=$?"
echo "DEMO|finished"
