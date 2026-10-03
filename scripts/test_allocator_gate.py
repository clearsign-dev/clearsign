import importlib.util
from pathlib import Path
import unittest

path = Path(__file__).resolve().parents[1] / "platform/grapheneos/check-deployment-tests.py"
spec = importlib.util.spec_from_file_location("allocator_gate", path)
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class NamedTest:
    def __init__(self, name):
        self.name = name

    def id(self):
        return self.name


class AllocatorGateTests(unittest.TestCase):
    def test_empty_suite_is_rejected(self):
        self.assertFalse(gate.deployment_passed(unittest.TestResult()))

    def test_only_known_cpp_assertion_failures_are_allowed(self):
        result = unittest.TestResult()
        result.testsRun = 50
        result.failures = [(NamedTest(name), "failure") for name in gate.ALLOWED_FAILURES]
        self.assertTrue(gate.deployment_passed(result))
        result.failures.append((NamedTest("test_smc.TestSimpleMemoryCorruption.test_write_after_free"), "failure"))
        self.assertFalse(gate.deployment_passed(result))

    def test_errors_and_skipped_tests_are_rejected(self):
        for field in ["errors", "skipped", "expectedFailures", "unexpectedSuccesses"]:
            result = unittest.TestResult()
            result.testsRun = 50
            setattr(result, field, [(NamedTest(next(iter(gate.ALLOWED_FAILURES))), "error")])
            self.assertFalse(gate.deployment_passed(result))
