
if (NOT EXISTS "/home/theinfinityglitch/Documents/Coding/rust/opengl_study/hello_triangle_indexed/target/debug/build/glfw-sys-ccbb8e2023740599/out/build/install_manifest.txt")
  message(FATAL_ERROR "Cannot find install manifest: \"/home/theinfinityglitch/Documents/Coding/rust/opengl_study/hello_triangle_indexed/target/debug/build/glfw-sys-ccbb8e2023740599/out/build/install_manifest.txt\"")
endif()

file(READ "/home/theinfinityglitch/Documents/Coding/rust/opengl_study/hello_triangle_indexed/target/debug/build/glfw-sys-ccbb8e2023740599/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")

foreach (file ${files})
  message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
  if (EXISTS "$ENV{DESTDIR}${file}")
    exec_program("/usr/bin/cmake" ARGS "-E remove \"$ENV{DESTDIR}${file}\""
                 OUTPUT_VARIABLE rm_out
                 RETURN_VALUE rm_retval)
    if (NOT "${rm_retval}" STREQUAL 0)
      MESSAGE(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif()
  elseif (IS_SYMLINK "$ENV{DESTDIR}${file}")
    EXEC_PROGRAM("/usr/bin/cmake" ARGS "-E remove \"$ENV{DESTDIR}${file}\""
                 OUTPUT_VARIABLE rm_out
                 RETURN_VALUE rm_retval)
    if (NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing symlink \"$ENV{DESTDIR}${file}\"")
    endif()
  else()
    message(STATUS "File \"$ENV{DESTDIR}${file}\" does not exist.")
  endif()
endforeach()

