use crate::util::{drawcolor, drawtext, Color};

const KEY_ACCEPT: i32 = '\n' as i32;
const KEY_REJECT: i32 = 27; // ESC Keycode
const KEY_PRINTABLE_START: i32 = 0x20;
const KEY_PRINTABLE_END: i32 = 0x7e;
const WND_WIDTH: i32 = 30;

pub fn textbox_open(title: &str) -> String {
    textbox_internal(title, None)
}

pub fn textbox_select(title: &str, items: &Vec<String>) -> String {
    textbox_internal(title, Some(&items))
}

fn textbox_internal(title: &str, items: Option<&Vec<String>>) -> String {
    let win = ncurses::subwin(ncurses::stdscr(), ncurses::LINES() - 1, WND_WIDTH, 0, 0);
    let mut text = String::new();
    loop {
        ncurses::werase(win);
        ncurses::wborder(win, 32, 32, 0, 32, 0, 0, 0, 0);
        drawtext(win, 1, 1, title, Color::Yellow, true, true, false, false, WND_WIDTH - 2);
        drawcolor(win, 2, 1, text.as_str(), Color::White, WND_WIDTH - 2);

        let mut pos = 3;
        let mut selvalue: Option<&String> = None;
        if items.is_some() {
            for item in items
                .unwrap()
                .iter()
                .filter(|x| x.to_lowercase().contains(&text.to_lowercase()))
            {
                if pos == 3 {
                    selvalue = Some(&item);
                    ncurses::wattron(win, ncurses::A_REVERSE());
                }
                ncurses::mvwaddnstr(win, pos, 1, item.as_str(), WND_WIDTH);
                if pos == 3 {
                    ncurses::wattroff(win, ncurses::A_REVERSE());
                }
                pos += 1;
                if pos > ncurses::LINES() - 2 {
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
