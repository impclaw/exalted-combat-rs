use ncurses;
mod mainwindow;
mod textbox;
mod util;
mod combat;

use util::Color;
use util::Drawable;

fn main() {
    ncurses::initscr();
    ncurses::noecho();
    ncurses::set_escdelay(0);
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::start_color();
    ncurses::init_pair(Color::White as i16, ncurses::COLOR_WHITE, 0);
    ncurses::init_pair(Color::Red as i16, ncurses::COLOR_RED, 0);
    ncurses::init_pair(Color::Blue as i16, ncurses::COLOR_BLUE, 0);
    ncurses::init_pair(Color::Green as i16, ncurses::COLOR_GREEN, 0);
    ncurses::init_pair(Color::Yellow as i16, ncurses::COLOR_YELLOW, 0);
    ncurses::init_pair(Color::Magenta as i16, ncurses::COLOR_MAGENTA, 0);

    let mut window = mainwindow::MainWindow::new();

    loop {
        window.render();
        window.refresh();

        let input = ncurses::getch();
        window.process_events(input);

        if input == 'q' as i32 {
            break;
        }
    }

    ncurses::endwin();
}
