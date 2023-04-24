use crate::util::Drawable;
use crate::util::Color;
use crate::util::Character;

const HELPSTR: &str = "a.dd d.ecis w.ith r.emov i.nit o.nsl";

const KEY_QUIT: i32 = 'q' as i32;
const KEY_UP: i32 = 'k' as i32;
const KEY_DOWN: i32 = 'j' as i32;
const KEY_NEW_ROUND: i32 = 'n' as i32;
const KEY_HEALTH: i32 = 'h' as i32;
const KEY_ONSLAUGHT: i32 = 'o' as i32;
const KEY_INITIATIVE: i32 = 'i' as i32;
const KEY_MARK_DONE: i32 = 'D' as i32;
const KEY_ADD_CHAR: i32 = 'a' as i32;
const KEY_ADD_MONSTER: i32 = 'm' as i32;
const KEY_DECISIVE_ATTACK:i32 = 'd' as i32;
const KEY_WITHERING_ATTACK:i32 = 'w' as i32;
const KEY_REMOVE:i32 = 'r' as i32;
const KEY_RESET:i32 = 'x' as i32;

pub struct MainWindow {
    leftwin: *mut i8, 
    rightwin: *mut i8, 
    logwin: *mut i8, 
    selpos: i32, 
    markedpos: i32,
    characters: Vec<Character>, 
    monsters: Vec<Character>, 
}

fn drawrt(win:*mut i8, y:i32, x:i32, text:&str, color:Color, 
    bold:bool, underline:bool, reverse:bool, dim:bool, len:i32) {
    if bold { ncurses::wattron(win, ncurses::A_BOLD()); }
    if underline { ncurses::wattron(win, ncurses::A_UNDERLINE()); }
    if reverse { ncurses::wattron(win, ncurses::A_REVERSE()); }
    if dim { ncurses::wattron(win, ncurses::A_DIM()); }

    ncurses::wattron(win, ncurses::COLOR_PAIR(color as i16));
    ncurses::mvwaddnstr(win, y, x, text, len);
    ncurses::wattroff(win, ncurses::COLOR_PAIR(color as i16));

    if bold { ncurses::wattroff(win, ncurses::A_BOLD()); }
    if underline { ncurses::wattroff(win, ncurses::A_UNDERLINE()); }
    if reverse { ncurses::wattroff(win, ncurses::A_REVERSE()); }
    if dim { ncurses::wattroff(win, ncurses::A_DIM()); }
}

fn drawcolor(win:*mut i8, y:i32, x:i32, text:&str, color:Color, len:i32) {
    drawrt(win, y, x, text, color, false, false, false, false, len);
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let midw = ncurses::COLS() / 2;
        let midh = ncurses::LINES() / 2;
        let monster_list:Vec<Character> = serde_json::from_str(
            std::fs::read_to_string("monsters.json").expect(
                "Could not open monsters.json"
            ).as_str()
        ).expect(
            "monsters.json has invalid formatting"
        );
        MainWindow { 
            leftwin: ncurses::subwin(ncurses::stdscr(), midh, midw, 0, 0), 
            rightwin: ncurses::subwin(ncurses::stdscr(), ncurses::LINES(), midw, 0, midw), 
            logwin: ncurses::subwin(ncurses::stdscr(), midh, midw, midh, 0), 
            characters: Character::defaults(), 
            monsters: monster_list,
            selpos: 1, 
            markedpos: -1,
        }
    }

    fn cursor_move(&mut self, amount:i32) {
        self.selpos += amount;
        if self.selpos > self.characters.len() as i32 {
            self.selpos = self.characters.len() as i32;
        } else if self.selpos < 1 {
            self.selpos = 1
        }
    }

    fn mark_done(&mut self) {
        self.characters[self.selpos as usize - 1].done ^= true;
        self.update()
    }

    fn set_char_initiative(&mut self) {
        let mut char = &mut self.characters[self.selpos as usize - 1];
        let result = crate::textbox::textbox_open("Initiative: ", 0, 0, 30);
        char.initiative = result.parse::<i32>().unwrap_or(char.initiative);
        self.update();
    }

    fn set_char_onslaught(&mut self) {
        let mut char = &mut self.characters[self.selpos as usize - 1];
        let result = crate::textbox::textbox_open("Onslaught: ", 0, 0, 30);
        char.onslaught = result.parse::<i32>().unwrap_or(char.onslaught);
        self.update();
    }

    fn set_char_health(&mut self) {
        let mut char = &mut self.characters[self.selpos as usize - 1];
        let result = crate::textbox::textbox_open("Health: ", 0, 0, 30);
        char.health = result.parse::<i32>().unwrap_or(char.health);
        self.update();
    }

    fn new_round(&mut self) {
        for char in &mut self.characters {
            char.done = false;
        }
        self.update();
    }

    fn add_char(&mut self) {
        let name = crate::textbox::textbox_open("Name: ", 0, 0, 30);
        if name == "" {
            return;
        }
        let initiative = crate::textbox::textbox_open("Join Battle Dice: ", 0, 0, 30);
        let char = Character::new(name, initiative.parse::<i32>().unwrap_or(0), 7);
        self.characters.push(char);
    }

    fn add_monster(&mut self) {
        let list:Vec<String> = self.monsters.iter().map(|x| x.name.clone()).collect();
        let selmonster = crate::textbox::textbox_select("Monster: ", 0, 0, 30, &list);
        let count = self.characters.iter().filter(|x| x.name == selmonster).count();
        let label = char::from_u32(count as u32 + 65);
        for monster in self.monsters.iter() {
            if monster.name == selmonster {
                let mut monster_copy = monster.clone();
                if label.is_some() {
                    monster_copy.label = Some(label.unwrap().to_string());
                }
                monster_copy.reset();
                self.characters.push(monster_copy);
            }
        }
        self.update();
    }

    fn update(&mut self) {
        self.characters.sort_by_key(|c| c.sortkey());
    }

    fn draw_char_list(&self) {
        ncurses::werase(self.leftwin);
        ncurses::wborder(self.leftwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.leftwin, 0, 2, "Participants", Color::White, true, true, false, false, 32);
        let mut pos:i32 = 1;
        for char in self.characters.iter() {
            let color = if self.markedpos == pos - 1 {
                Color::Blue
            } else if char.dead() {
                Color::Red
            } else if char.crashed() {
                Color::Yellow
            } else {
                Color::White
            };

            drawrt(self.leftwin, pos, 2, 
                format!(
                    "{:<width$}{:<4}{:<4}{:<2}{:<2}{:<13}", 
                    format!("{} {}", char.name, char.label.clone().unwrap_or(String::from(""))), 
                    char.initiative, 
                    char.onslaught, 
                    if char.done { "D" } else { "" }, 
                    if char.crashed() { "C" } else { "" }, 
                    format!("{}/{}", char.health, char.maxhealth), 
                    width = (ncurses::COLS() / 2 - 29) as usize)
                .as_str(), 
                color, false, false, pos == self.selpos, char.done, ncurses::COLS() / 2);
            pos += 1;
        }

        drawcolor(self.leftwin, ncurses::LINES() / 2 - 1, 2, HELPSTR, Color::White, ncurses::COLS() / 2 - 4);
    }

    fn draw_details(&self) { 
        ncurses::werase(self.rightwin);
        ncurses::wborder(self.rightwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.rightwin, 0, 2, "Details", Color::White, true, true, false, false, 32);
        
        let char = match self.characters.get(self.selpos as usize - 1) {
            Some(x) => x,
            None => { return; }
        };
        
        drawrt(self.rightwin, 1, 2, &char.name, 
            Color::Green, true, false, false, false, ncurses::COLS() / 2 - 2
        );
        drawcolor(self.rightwin, 2, 2, 
            format!("Evasion: {}", char.evasion).as_str(), 
            Color::Blue, ncurses::COLS() / 4 - 2
        );
        drawcolor(self.rightwin, 2, ncurses::COLS() / 4 - 1, 
            format!("Parry:    {}", char.parry).as_str(), 
            Color::Blue, ncurses::COLS() / 4 - 2
        );
        drawcolor(self.rightwin, 3, 2, 
            format!("Soak:    {}", char.soak).as_str(), 
            Color::Blue, ncurses::COLS() / 4 - 2
        );
        drawcolor(self.rightwin, 3, ncurses::COLS() / 4 - 1, 
            format!("Hardness: {}", char.hardness.unwrap_or(0)).as_str(), 
            Color::Blue, ncurses::COLS() / 4 - 2
        );

        
    }
    
    fn draw_log(&self) {
        ncurses::werase(self.logwin);
        ncurses::wborder(self.logwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawrt(self.logwin, 0, 2, "Combat Log", Color::White, true, true, false, false, 32);
    }

}

impl Drawable for MainWindow {
    fn refresh(&self) {
        ncurses::wrefresh(self.rightwin);
        ncurses::wrefresh(self.leftwin);
        ncurses::wrefresh(self.logwin);
    }

    fn process_events(&mut self, ch:i32) {
        match ch {
            KEY_UP => self.cursor_move(-1),
            KEY_DOWN => self.cursor_move(1),
            ncurses::KEY_UP => self.cursor_move(-1), 
            ncurses::KEY_DOWN => self.cursor_move(1),
            KEY_MARK_DONE => self.mark_done(),
            KEY_INITIATIVE => self.set_char_initiative(),
            KEY_ONSLAUGHT => self.set_char_onslaught(), 
            KEY_HEALTH => self.set_char_health(),
            KEY_NEW_ROUND => self.new_round(),
            KEY_ADD_CHAR => self.add_char(),
            KEY_ADD_MONSTER => self.add_monster(),
            _ => {}, 
        }
    }

    fn render(&self) {
        self.draw_char_list();
        self.draw_details();
        self.draw_log();
    }
}
