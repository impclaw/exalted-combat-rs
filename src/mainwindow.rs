use crate::util::Drawable;
use crate::util::Color;
use crate::util::Character;

pub struct MainWindow {
    leftwin: *mut i8, 
    rightwin: *mut i8, 
    logwin: *mut i8, 
    selpos: i32, 
    characters: Vec<Character>, 
}

fn drawrt(win:*mut i8, y:i32, x:i32, text:&str, color:Color, bold:bool, underline:bool, reverse:bool, len:i32) {
    if bold {
        ncurses::wattron(win, ncurses::A_BOLD());
    }
    if underline {
        ncurses::wattron(win, ncurses::A_UNDERLINE());
    }
    if reverse {
        ncurses::wattron(win, ncurses::A_REVERSE());
    }
    ncurses::wattron(win, ncurses::COLOR_PAIR(color as i16));
    ncurses::mvwaddnstr(win, y, x, text, len);
    ncurses::wattroff(win, ncurses::COLOR_PAIR(color as i16));
    if bold {
        ncurses::wattroff(win, ncurses::A_BOLD());
    }
    if underline {
        ncurses::wattroff(win, ncurses::A_UNDERLINE());
    }
    if reverse {
        ncurses::wattroff(win, ncurses::A_REVERSE());
    }
}

fn drawcolor(win:*mut i8, y:i32, x:i32, text:&str, color:Color, len:i32) {
    drawrt(win, y, x, text, color, false, false, false, len);
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let midw = ncurses::COLS() / 2;
        let midh = ncurses::LINES() / 2;
        MainWindow { 
            leftwin: ncurses::subwin(ncurses::stdscr(), midh, midw, 0, 0), 
            rightwin: ncurses::subwin(ncurses::stdscr(), ncurses::LINES(), midw, 0, midw), 
            logwin: ncurses::subwin(ncurses::stdscr(), midh, midw, midh, 0), 
            characters: Character::defaults(), 
            selpos: 1, 
        }
    }
}

impl Drawable for MainWindow {
    fn refresh(&self) {
        ncurses::wrefresh(self.rightwin);
        ncurses::wrefresh(self.leftwin);
        ncurses::wrefresh(self.logwin);
    }

    fn render(&self) {
        ncurses::werase(self.rightwin);
        ncurses::wborder(self.rightwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.rightwin, 0, 2, "Details", Color::White, true, true, false, 32);

        ncurses::werase(self.leftwin);
        ncurses::wborder(self.leftwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.leftwin, 0, 2, "Participants", Color::White, true, true, false, 32);

        ncurses::werase(self.logwin);
        ncurses::wborder(self.logwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.logwin, 0, 2, "Combat Log", Color::White, true, true, false, 32);

        let mut pos:i32 = 1;
        for char in self.characters.iter() {
            drawrt(self.leftwin, pos, 2, 
                format!(
                    "{:<width$}{:<4}{:<4}{:<13}", 
                    char.name, 
                    char.initiative, 
                    char.onslaught, 
                    "Hello", 
                    width = (ncurses::COLS() / 2 - 25) as usize)
                .as_str(), 
                Color::White, false, false, pos == self.selpos, ncurses::COLS() / 2);
            pos += 1;
        }
    }
}
