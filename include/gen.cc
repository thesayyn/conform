
#include "conformance/binary_json_conformance_suite.h"
#include "conformance/conformance_test.h"
#include "conformance/text_format_conformance_suite.h"
#include "conformance/conformance.pb.h"
#include "runner.h"
#include "gen.h"

#include <string>
#include <vector>
#include "conformance/conformance.pb.h"

template <typename... Args> inline void unused(Args&&...) {}

std::vector<Case> cases = std::vector<Case>();

void google::protobuf::ConformanceTestSuite::RunValidBinaryInputTest(const ConformanceRequestSetting &setting,
                                                                     const std::string &equivalent_wire_format, bool require_same_wire_format)
{
    std::string payload;
    setting.GetRequest().SerializeToString(&payload);
    Case _case = Case(
        setting.GetTestName(),
        payload,
        setting.ConformanceLevelToString(setting.GetLevel()),
        setting.GetSyntaxIdentifier(),
        equivalent_wire_format,
        require_same_wire_format);
    cases.push_back(_case);
}

bool google::protobuf::ConformanceTestSuite::RunSuite(ConformanceTestRunner *runner,
                                                      std::string *output,
                                                      const std::string &filename,
                                                      conformance::FailureSet *failure_list)
{
    unused(output, filename, failure_list);
    runner_ = runner;
    successes_ = 0;
    expected_failures_ = 0;
    skipped_.clear();
    test_names_.clear();
    unexpected_failing_tests_.clear();
    unexpected_succeeding_tests_.clear();
    RunSuiteImpl();
    return true;
}

std::vector<Case> extract_suite()
{
    google::protobuf::BinaryAndJsonConformanceSuite binary_and_json_suite;
    NoopRunner runner;

    std::string output;
    std::string failure_list_filename = "/dev/null";
    conformance::FailureSet failure_list;
    binary_and_json_suite.RunSuite(&runner, &output, failure_list_filename,
                                   &failure_list);
    std::vector<Case> _cases = cases;
    return _cases;
}
