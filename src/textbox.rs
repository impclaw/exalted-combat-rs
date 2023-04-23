const KEY_ACCEPT: i32 = '\n' as i32;
const KEY_REJECT: i32 = 27; // ESC Keycode
const KEY_PRINTABLE_START: i32 = 0x20;
const KEY_PRINTABLE_END: i32 = 0x7e;

pub fn textbox_open(title: &str, x:i32, y:i32, w:i32) -> String {
    const h: i32 = 4;
    let win = ncurses::subwin(ncurses::stdscr(), h, w, y, x);
    let mut text = String::new();
    let mut curpos = 0;
    loop {
        ncurses::werase(win);
        ncurses::wborder(win, 32, 32, 0, 32, 0, 0, 0, 0);
        ncurses::wattron(win, ncurses::A_UNDERLINE());
        ncurses::mvwaddnstr(win, y + 1, x + 1, title, w - 2);
        ncurses::wattroff(win, ncurses::A_UNDERLINE());
        ncurses::mvwaddnstr(win, y + 2, x + 1, text.as_str(), w - 2);
        ncurses::wrefresh(win);
        let input = ncurses::getch();
        if input == KEY_ACCEPT {
            return text;
        } else if input == ncurses::KEY_BACKSPACE {
            text.pop();
        } else if input == KEY_REJECT {
            text.clear();
            return text;
        } else if input >= KEY_PRINTABLE_START && input <= KEY_PRINTABLE_END { 
            text.push(char::from_u32(input as u32).unwrap_or('.')); 
        }
    }
}

