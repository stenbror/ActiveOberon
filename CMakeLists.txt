cmake_minimum_required(VERSION 3.14...3.27)

set(CMAKE_CXX_COMPILER "/usr/bin/clang++")
set(CMAKE_CXX_STANDARD_REQUIRED "c++-20")

set(target_include_directorie ./)
set(CMAKE_CXX_FLAGS "-I.. -std=c++20")


project(
  ActiveOberon
  VERSION 0.1
  DESCRIPTION "Active Oberon compiler"
  LANGUAGES CXX)

  set(HEADER_FILES *.h)

  add_executable(ActiveOberon main.cc scanner.cc parser.cc)

  if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)

  # Optionally set things like CMAKE_CXX_STANDARD, CMAKE_POSITION_INDEPENDENT_CODE here

  # Let's ensure -std=c++xx instead of -std=g++xx
  set(CMAKE_CXX_EXTENSIONS OFF)

  # Let's nicely support folders in IDEs
  set_property(GLOBAL PROPERTY USE_FOLDERS ON)

  # Testing only available if this is the main app
  # Note this needs to be done in the main CMakeLists
  # since it calls enable_testing, which must be in the
  # main CMakeLists.
  include(CTest)

  enable_testing()

  # Docs only available if this is the main app
  find_package(Doxygen)
  if(Doxygen_FOUND)
    add_subdirectory(docs)
  else()
    message(STATUS "Doxygen not found, not building docs")
  endif()
endif()

include(FetchContent)

if((CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME OR MODERN_CMAKE_BUILD_TESTING)
   AND BUILD_TESTING)
  #add_subdirectory(tests)
endif()
