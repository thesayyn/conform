if (protobuf_JSONCPP_PROVIDER STREQUAL "module")
  if (NOT EXISTS "${protobuf_SOURCE_DIR}/third_party/jsoncpp/CMakeLists.txt")
    message(FATAL_ERROR
            "Cannot find third_party/jsoncpp directory that's needed to "
            "build conformance tests. If you use git, make sure you have cloned "
            "submodules:\n"
            "  git submodule update --init --recursive\n"
            "If instead you want to skip them, run cmake with:\n"
            "  cmake -Dprotobuf_BUILD_CONFORMANCE=OFF\n")
  endif()
elseif(protobuf_JSONCPP_PROVIDER STREQUAL "package")
  find_package(jsoncpp REQUIRED)
endif()

set(protoc_cpp_args)
if (protobuf_BUILD_SHARED_LIBS)
  set(protoc_cpp_args "dllexport_decl=PROTOBUF_TEST_EXPORTS:")
endif ()

add_custom_command(
  OUTPUT
    ${protobuf_SOURCE_DIR}/conformance/conformance.pb.h
    ${protobuf_SOURCE_DIR}/conformance/conformance.pb.cc
  DEPENDS ${protobuf_PROTOC_EXE} ${protobuf_SOURCE_DIR}/conformance/conformance.proto
  COMMAND ${protobuf_PROTOC_EXE} ${protobuf_SOURCE_DIR}/conformance/conformance.proto
      --proto_path=${protobuf_SOURCE_DIR}/conformance
      --cpp_out=${protoc_cpp_args}${protobuf_SOURCE_DIR}/conformance
)

add_custom_command(
  OUTPUT
    ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.h
    ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.cc
    ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.h
    ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.cc
    ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.pb.h
    ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.pb.cc
    ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.pb.h
    ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.pb.cc
  DEPENDS ${protobuf_PROTOC_EXE}
          ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.proto
          ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.proto
          ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.proto
          ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.proto
  COMMAND ${protobuf_PROTOC_EXE}
              ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.proto
              ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.proto
              ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.proto
              ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.proto
            --proto_path=${protobuf_SOURCE_DIR}/src
            --cpp_out=${protoc_cpp_args}${protobuf_SOURCE_DIR}/src
)

add_library(conformance_common ${protobuf_SHARED_OR_STATIC}
  ${protobuf_SOURCE_DIR}/conformance/conformance.pb.h
  ${protobuf_SOURCE_DIR}/conformance/conformance.pb.cc
  ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.h
  ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.cc
  ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.h
  ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.cc
  ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.pb.h
  ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto3_editions.pb.cc
  ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.pb.h
  ${protobuf_SOURCE_DIR}/src/google/protobuf/editions/golden/test_messages_proto2_editions.pb.cc
  ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.cc
  ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.h
  ${protobuf_SOURCE_DIR}/conformance/conformance_test.cc
  ${protobuf_SOURCE_DIR}/conformance/conformance_test_runner.cc
  ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.cc
  ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.h
)
target_link_libraries(conformance_common
  ${protobuf_LIB_PROTOBUF}
  ${protobuf_ABSL_USED_TARGETS}
)
if(protobuf_BUILD_SHARED_LIBS)
  target_compile_definitions(conformance_common
    PUBLIC  PROTOBUF_USE_DLLS
    PRIVATE LIBPROTOBUF_TEST_EXPORTS)
endif()

target_include_directories(
  conformance_common
  PUBLIC ${protobuf_SOURCE_DIR} ${protobuf_SOURCE_DIR}/conformance)

set(JSONCPP_WITH_TESTS OFF CACHE BOOL "Disable tests")
if(protobuf_JSONCPP_PROVIDER STREQUAL "module")
  add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/third_party/jsoncpp third_party/jsoncpp)
  target_include_directories(conformance_common PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/third_party/jsoncpp/include)
  if(BUILD_SHARED_LIBS)
    target_link_libraries(conformance_common jsoncpp_lib)
  else()
    target_link_libraries(conformance_common jsoncpp_static)
  endif()
else()
  target_link_libraries(conformance_common jsoncpp)
endif()
