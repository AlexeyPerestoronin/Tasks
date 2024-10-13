include("${CMAKE_SOURCE_DIR}/FindAllSourceFiles.cmake")
include("${CMAKE_SOURCE_DIR}/ComposeFileBySourceGroup.cmake")


# brief: creates suit tests
# param: i_targetTestFile - target file for new suit-test
function(AddSuitTest i_targetTestFile)
    message("[add suit-test: ${i_targetTestFile}] begin")
    
    # place the suit-test in target VS-filter
    get_filename_component(fileDir  ${i_targetTestFile} DIRECTORY)
    string(REPLACE "${CMAKE_CURRENT_SOURCE_DIR}/suit-tests/" "" suitTestGroupe "${fileDir}")
    string(REPLACE "/" "-" suitTestPrefix "${suitTestGroupe}")
    
    get_filename_component(fileNameWithouExtension ${i_targetTestFile} NAME_WE)
    set(suitTestExeName ${suitTestPrefix}-${fileNameWithouExtension})
    add_executable(${suitTestExeName} ${i_targetTestFile} "test-main.cpp" "${fileDir}/${fileNameWithouExtension}.md")
    set_source_files_properties("${fileDir}/${fileNameWithouExtension}.md" PROPERTIES LABELS "Answer")
    set_property(TARGET ${suitTestExeName} PROPERTY CXX_STANDARD ${TASKS_LIB_CXX_STANDART})
    target_link_libraries(${suitTestExeName} GTest::gtest_main)
    target_link_libraries(${suitTestExeName} GTest::gmock_main)
    target_link_libraries(${suitTestExeName} ${TASKS_LIB_NAME})
    
    set_target_properties(${suitTestExeName} PROPERTIES FOLDER "Tests/SuitTests/${suitTestGroupe}")

    source_group("Task" FILES "${fileDir}/${fileNameWithouExtension}.md" "${fileDir}/${fileNameWithouExtension}.cpp")
    source_group("Src" FILES "test-main.cpp")

    message("[add suit-test: ${i_targetTestFile}] end")
endfunction(AddSuitTest)


# brief: creates suit tests
# note1: one suit test is creates from one code file and presents as unique project
function(AddSuitTests)
    message("[AddSuitTests] begin")

    file(GLOB_RECURSE sources_list "${CMAKE_CURRENT_SOURCE_DIR}/suit-tests/*")
    foreach(source ${sources_list})
        if(NOT IS_DIRECTORY ${source})
            get_filename_component(sourceExt ${source} EXT)
            if(${sourceExt} STREQUAL ".cpp")
                AddSuitTest(${source})
            endif()
        endif()
    endforeach()

    message("[AddSuitTests] end")
endfunction(AddSuitTests)
