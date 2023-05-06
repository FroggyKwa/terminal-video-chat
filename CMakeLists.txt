cmake_minimum_required(VERSION 3.23)
set(CMAKE_CXX_STANDARD 23)
project(AsciiVideoChat)

include_directories(.)

add_executable(AsciiVideoChat src/main.cpp include/term_output.hpp)

set(CURSES_NEED_NCURSES TRUE)
find_package(Curses REQUIRED)

if (CURSES_FOUND AND CURSES_HAVE_NCURSES_H)
    include_directories(${CURSES_INCLUDE_DIR})
    target_link_libraries (AsciiVideoChat ${CURSES_LIBRARIES})
endif()
