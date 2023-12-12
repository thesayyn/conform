#include <errno.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <future>

#include "conformance/conformance.pb.h"
#include "conformance/conformance_test.h"
#include "runner.h"

using conformance::ConformanceResponse;
using google::protobuf::ConformanceTestSuite;
using std::string;

template <typename... Args> inline void unused(Args&&...) {}


void NoopRunner::RunTest(const std::string &name,
                         const std::string &req,
                         std::string *res)
{
    unused(name, req, res);
   // std::cout << name << std::endl;
}