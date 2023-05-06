#include <ncurses.h>
#include <include/term_output.hpp>

int main(int argc, char *argv[]) {
    output::init();
    output::toggle_screen_update();
    //F9 - exit
    return 0;
}