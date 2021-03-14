set(INCLUDES_BASE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../include)

function(set_as_hdll target)
    set_target_properties(${target}.hdll
        PROPERTIES
        PREFIX ""
        OUTPUT_NAME ${target}
        SUFFIX .hdll
    )
endfunction()



option(WITH_SQLITE "Build sqlite.hdll." ON)
if(WITH_SQLITE)
    add_subdirectory(sqlite)
endif()


option(WITH_UV "Build uv.hdll." ON)
if(WITH_UV)
    add_subdirectory(uv)
endif()

