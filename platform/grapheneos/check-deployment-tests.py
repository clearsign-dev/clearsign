"""Run upstream tests, allowing only the disabled C++ allocator checks to fail."""

from pathlib import Path
import os
import sys
import unittest

ALLOWED_FAILURES = {
    "test_smc.TestSimpleMemoryCorruption.test_delete_type_size_mismatch",
    "test_smc.TestSimpleMemoryCorruption.test_invalid_aligned_sized_delete_large",
    "test_smc.TestSimpleMemoryCorruption.test_invalid_aligned_sized_delete_small",
}


def deployment_passed(result):
    return (
        result.testsRun > len(ALLOWED_FAILURES)
        and not result.errors
        and not result.skipped
        and not result.expectedFailures
        and not result.unexpectedSuccesses
        and all(test.id() in ALLOWED_FAILURES for test, _ in result.failures)
    )


if __name__ == "__main__":
    os.chdir(Path(sys.argv[1]).resolve())
    suite = unittest.defaultTestLoader.discover("test")
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(0 if deployment_passed(result) else 1)
