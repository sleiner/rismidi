cmake_minimum_required(VERSION 3.23)

project(sdk_wrapper)

# Import the SDK
include(FetchContent)
FetchContent_Declare(
  vst3sdk
  GIT_REPOSITORY https://github.com/steinbergmedia/vst3sdk.git
  GIT_TAG 05c4a9763f311a58fe3f34b68a199e5eea92e2b4 # v3.7.6
)

message(STATUS "Fetching VST3 SDK")
FetchContent_Populate(vst3sdk)

message(STATUS "Configuring VST3 SDK")
list(APPEND CMAKE_MESSAGE_INDENT "  ")
add_subdirectory(${vst3sdk_SOURCE_DIR} ${vst3sdk_BINARY_DIR} EXCLUDE_FROM_ALL)
list(POP_BACK CMAKE_MESSAGE_INDENT)

install(TARGETS validator RUNTIME)

# To ensure that the validator target is built before installation, we add it as
# dependency of this dummy target.
add_custom_target(targets_to_install ALL)
add_dependencies(targets_to_install validator)
