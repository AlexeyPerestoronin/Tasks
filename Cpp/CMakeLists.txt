cmake_minimum_required(VERSION 3.1.0)

project(Tasks)

include("${CMAKE_SOURCE_DIR}/cmake/AddGoogleTests.cmake")
AddGoogleTests()

include("${CMAKE_SOURCE_DIR}/cmake/AddSuitTests.cmake")
# AddSuitTests("${CMAKE_SOURCE_DIR}/src/OpenSources/Easy")
AddSuitTests("${CMAKE_SOURCE_DIR}/src/OpenSources/LetCode")
