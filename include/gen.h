#pragma once

#include <string>
#include <vector>

class Case
{
public:
    Case(
        std::string name,
        std::string payload,
        int level,
        std::string syntax,
        std::string equivalent,
        bool require_same_wire_format,
        std::string assert_by)
        : name{name},
          payload{payload},
          level{level},
          syntax{syntax},
          equivalent{equivalent},
          require_same_wire_format{require_same_wire_format},
          assert_by{assert_by}
    {
    }
    std::string get_name();
    std::string get_payload();
    std::string get_equivalent();
    int get_level();
    std::string get_syntax();
    bool get_require_same_wire_format();
    std::string get_assert_by();

private:
    std::string name;
    std::string payload;
    int level;
    std::string syntax;
    std::string equivalent;
    bool require_same_wire_format;
    std::string assert_by;
};

std::string Case::get_name()
{
    return this->name;
}

std::string Case::get_payload()
{
    return this->payload;
}

int Case::get_level()
{
    return this->level;
}

std::string Case::get_syntax()
{
    return this->syntax;
}
std::string Case::get_equivalent()
{
    return this->equivalent;
}

bool Case::get_require_same_wire_format()
{
    return this->require_same_wire_format;
}

std::string Case::get_assert_by()
{
    return this->assert_by;
}

extern std::vector<Case> cases;

extern std::vector<Case> extract_suite();