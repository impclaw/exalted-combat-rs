use rand::Rng;

#[derive(Clone, Copy)]
pub enum Color {
    White = 1,
    Red = 2,
    Blue = 3,
    Green = 4,
    Yellow = 5,
    Magenta = 6,
}

pub trait Drawable {
    fn refresh(&self);
    fn render(&self);
    fn process_events(&mut self, ch: i32);
}

pub fn drawtext(
    win: *mut i8, y: i32, x: i32, text: &str, color: Color, bold: bool, underline: bool,
    reverse: bool, dim: bool, len: i32,
) {
    if bold {
        ncurses::wattron(win, ncurses::A_BOLD());
    }
    if underline {
        ncurses::wattron(win, ncurses::A_UNDERLINE());
    }
    if reverse {
        ncurses::wattron(win, ncurses::A_REVERSE());
    }
    if dim {
        ncurses::wattron(win, ncurses::A_DIM());
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
    if dim {
        ncurses::wattroff(win, ncurses::A_DIM());
    }
}

pub fn drawcolor(win: *mut i8, y: i32, x: i32, text: &str, color: Color, len: i32) {
    drawtext(win, y, x, text, color, false, false, false, false, len);
}

//Rolls an exalted die roll, ignoring 1s
pub fn roll_dice(count: i32) -> i32 {
    let mut result: i32 = 0;
    let mut botches: i32 = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        result += match rng.gen_range(1..=10) {
            7..=9 => 1,
            10 => 2,
            1 => {
                botches += 1;
                0
            }
            _ => 0,
        };
    }
    if result > 0 {
        return result;
    } else {
        return -botches;
    }
}
