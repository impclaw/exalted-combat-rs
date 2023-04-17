use ncurses;
mod mainwindow;
mod util;

use util::Drawable;
use util::Color;

fn main() {
    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::start_color();
    ncurses::init_pair(Color::White as i16, ncurses::COLOR_WHITE, 0);
    ncurses::init_pair(Color::Red as i16, ncurses::COLOR_RED, 0);
    ncurses::init_pair(Color::Blue as i16, ncurses::COLOR_BLUE, 0);
    ncurses::init_pair(Color::Green as i16, ncurses::COLOR_GREEN, 0);

    let window = mainwindow::MainWindow::new();

    window.render();
    window.refresh();

    ncurses::getch();
    ncurses::endwin();
}
