cmake_minimum_required(VERSION 3.14)

include(FetchContent)

# brief: add Google tests to current project
function(AddGoogleTests)
    message("[AddGoogleTests] begin")

    FetchContent_Declare(
        googletest
        GIT_REPOSITORY https://github.com/google/googletest.git
        GIT_TAG release-1.12.1
        GIT_PROGRESS true
    )

    FetchContent_MakeAvailable(googletest)

    message("[AddGoogleTests] end")
endfunction(AddGoogleTests)
