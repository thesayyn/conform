#include "conformance/conformance_test.h"
#include "gen.h"

using google::protobuf::ConformanceTestRunner;

class NoopRunner : public ConformanceTestRunner
{
public:
    void RunTest(const std::string &test_name, const std::string &request,
                 std::string *response);
private:

};