#ifndef ASCIIVIDEOCHAT_TERM_OUTPUT_HPP
#define ASCIIVIDEOCHAT_TERM_OUTPUT_HPP

#pragma once

#include <ncurses.h>

namespace output {
    void init() {
        initscr();
        keypad(stdscr, TRUE);
        raw();
    }

    void toggle_screen_update() {
        int col;
        int row;
        getmaxyx(stdscr, row, col);
        char buffer[row][col];
        int ch;
        move(0, 0);
        while ((ch = getch()) != KEY_F(9)) {
            for (int i = 0; i < row; ++i) {
                for (int j = 0; j < col; ++j) {
                    buffer[i][j] = ch;
                    addch(buffer[i][j]);
                }
            }
            move(0, 0);
        }
    }
}
#endif
