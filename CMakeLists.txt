cmake_minimum_required(VERSION 3.14...3.27)

set(CMAKE_CXX_COMPILER "/usr/bin/clang++")
set(CMAKE_CXX_STANDARD_REQUIRED "c++-20")

set(target_include_directorie ./)
set(CMAKE_CXX_FLAGS "-I.. -std=c++20")




# Add a new unit test
# A new target with the test sources is constructed, and a CTest test with the
# same name is created. The new test is also added to the global list of test
# contained in the check target
macro(add_unit_test)
    set(options PARALLEL)
    set(oneValueArgs NAME)
    set(multiValueArgs SOURCES LIBS)
    cmake_parse_arguments(TEST "${options}" "${oneValueArgs}"
                               "${multiValueArgs}" ${ARGN})
    message(STATUS "generating ${TEST_NAME}")
    add_executable(${TEST_NAME} EXCLUDE_FROM_ALL ${TEST_SOURCES})
    set_target_properties(${TEST_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY
                          ${PROJECT_BINARY_DIR}/rundir/test)
    target_link_libraries(${TEST_NAME} ${TEST_LIBS})

    if(TEST_PARALLEL AND HAVE_MPI)
      set(TESTCOMMAND ${MPIEXEC})
      set(TESTARGS ${MPIEXEC_NUMPROC_FLAG} 3 ${MPIEXEC_PREFLAGS}
                   "./${TEST_NAME}" ${MPIEXEC_POSTFLAGS})
      set(TESTCOMMAND ${TESTCOMMAND} ${TESTARGS})
    else(TEST_PARALLEL AND HAVE_MPI)
      set(TESTCOMMAND ${TEST_NAME})
    endif(TEST_PARALLEL AND HAVE_MPI)
    add_test(NAME ${TEST_NAME}
             WORKING_DIRECTORY ${PROJECT_BINARY_DIR}/rundir/test
             COMMAND ${TESTCOMMAND})

    set(TESTNAMES ${TESTNAMES} ${TEST_NAME})
endmacro(add_unit_test)


add_custom_target(check_${TEST_NAME}
                  COMMAND ${CMAKE_CTEST_COMMAND}
                          -R ${TEST_NAME}
                  DEPENDS ${TEST_NAME})



project(
  ActiveOberon
  VERSION 0.1
  DESCRIPTION "Active Oberon compiler"
  LANGUAGES CXX)

  set(HEADER_FILES *.h)

  add_executable(ActiveOberon main.cc scanner.cc parser.cc)

  add_subdirectory (tests)

  
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





  # Add list of UnitTests executables in this projects
  add_test (NAME TestScannerOperators COMMAND tests/TestCompilerScanner_Operators)
  add_test (NAME TestScannerReservedKeywords COMMAND tests/TestCompilerScanner_ReservedKeywords)

  # Finished adding test executables.





  add_custom_target(check COMMAND ${CMAKE_CTEST_COMMAND}
                        DEPENDS ${TESTNAMES})

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
