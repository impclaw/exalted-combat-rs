const KEY_ACCEPT: i32 = '\n' as i32;
const KEY_REJECT: i32 = 27; // ESC Keycode
const KEY_PRINTABLE_START: i32 = 0x20;
const KEY_PRINTABLE_END: i32 = 0x7e;

pub fn textbox_open(title: &str, x:i32, y:i32, w:i32) -> String {
    textbox_internal(title, x, y, w, None)
}

pub fn textbox_select(title: &str, x:i32, y:i32, w:i32, items:&Vec<String>) -> String {
    textbox_internal(title, x, y, w, Some(&items))
}

fn textbox_internal(title: &str, x:i32, y:i32, w:i32, items:Option<&Vec<String>>) -> String {
    let h: i32 = ncurses::LINES() - y - 1;
    let win = ncurses::subwin(ncurses::stdscr(), h, w, y, x);
    let mut text = String::new();
    let mut selvalue:Option<&String> = None;
    loop {
        ncurses::werase(win);
        ncurses::wborder(win, 32, 32, 0, 32, 0, 0, 0, 0);
        ncurses::wattron(win, ncurses::A_UNDERLINE());
        ncurses::mvwaddnstr(win, y + 1, x + 1, title, w - 2);
        ncurses::wattroff(win, ncurses::A_UNDERLINE());
        ncurses::mvwaddnstr(win, y + 2, x + 1, text.as_str(), w - 2);

        let mut pos = 3;
        selvalue = None;
        if items.is_some() {
            for item in items.unwrap().iter().filter(|x| x.to_lowercase().contains(&text.to_lowercase())) {
                if pos == 3 {
                    selvalue = Some(&item);
                    ncurses::wattron(win, ncurses::A_REVERSE());
                }
                ncurses::mvwaddnstr(win, y + pos, x + 1, item.as_str(), w);
                if pos == 3 {
                    ncurses::wattroff(win, ncurses::A_REVERSE());
                }
                pos += 1;
                if pos > ncurses::LINES() - y - 2 {
                    break;
                }
            }
        }

        ncurses::wrefresh(win);
        let input = ncurses::getch();
        if input == KEY_ACCEPT && selvalue.is_none() {
            return text;
        } else if input == KEY_ACCEPT && selvalue.is_some() {
            return selvalue.unwrap().clone();
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

