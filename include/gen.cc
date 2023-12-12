
#include "conformance/conformance_test.h"
#include "conformance/conformance.pb.h"
#include "conformance/text_format_conformance_suite.h"
#include "conformance/binary_json_conformance_suite.h"
#include "runner.h"
#include "gen.h"

#include <string>
#include <vector>

template <typename... Args>
inline void unused(Args &&...) {}

std::vector<Case> cases = std::vector<Case>();

using google::protobuf::ConformanceTestSuite;

void google::protobuf::ConformanceTestSuite::RunValidBinaryInputTest(const ConformanceRequestSetting &setting,
                                                                     const std::string &equivalent_wire_format, bool require_same_wire_format)
{
    std::string payload;
    setting.GetRequest().SerializeToString(&payload);
    Case _case = Case(
        setting.GetTestName(),
        payload,
        setting.GetLevel(),
        setting.GetSyntaxIdentifier(),
        equivalent_wire_format,
        require_same_wire_format,
        "equivalence");
    cases.push_back(_case);
}

void google::protobuf::ConformanceTestSuite::ReportFailure(const std::string &test_name,
                                                           ConformanceLevel level,
                                                           const conformance::ConformanceRequest &request,
                                                           const conformance::ConformanceResponse &response,
                                                           absl::string_view message)
{

    std::string payload;
    request.SerializeToString(&payload);
    if (message == "Should have failed to parse, but didn't.")
    {
        Case _case = Case(
            test_name,
            payload,
            level,
            "proto3", // todo
            "",
            false,
            "f_parse");
        cases.push_back(_case);
    } else if (message == "Should have failed to serialize, but didn't.") {
        Case _case = Case(
            test_name,
            payload,
            level,
            "proto3", // todo
            "",
            false,
            "f_serialize");
        cases.push_back(_case);
    } else if (message == "Expected JSON payload but got type 0" && test_name.find(".Validator") != std::string::npos) {
        Case _case = Case(
            test_name,
            payload,
            level,
            "proto3", // todo
            "",
            false,
            "json_validator"
        );
        cases.push_back(_case);
    }
    else
    {
        std::cout << test_name << std::endl;
        std::cout << message << std::endl;
    }
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
    google::protobuf::TextFormatConformanceTestSuite text_suite;
    NoopRunner runner;

    std::string output;
    std::string failure_list_filename;
    conformance::FailureSet failure_list;
    binary_and_json_suite.RunSuite(&runner, &output, failure_list_filename,
                                   &failure_list);
    // text_suite.RunSuite(&runner, &output, failure_list_filename, &failure_list);
    std::vector<Case> _cases = cases;
    return _cases;
}
