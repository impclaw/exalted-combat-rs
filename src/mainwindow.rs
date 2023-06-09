use crate::combat::{Character, Encounter, MonsterDB};
use crate::textbox::{textbox_open, textbox_select};
use crate::util::Color;
use crate::util::Drawable;
use crate::util::{drawcolor, drawtext};

const HELPSTR: &str = "a.dd d.ecis w.ith r.emov i.nit o.nsl";

const KEY_UP: i32 = 'k' as i32;
const KEY_DOWN: i32 = 'j' as i32;
const KEY_NEW_ROUND: i32 = 'n' as i32;
const KEY_HEALTH: i32 = 'h' as i32;
const KEY_ONSLAUGHT: i32 = 'o' as i32;
const KEY_INITIATIVE: i32 = 'i' as i32;
const KEY_MARK_DONE: i32 = 'D' as i32;
const KEY_ADD_CHAR: i32 = 'a' as i32;
const KEY_ADD_MONSTER: i32 = 'm' as i32;
const KEY_DECISIVE_ATTACK: i32 = 'd' as i32;
const KEY_WITHERING_ATTACK: i32 = 'w' as i32;
const KEY_REMOVE: i32 = 'r' as i32;
const KEY_RESET: i32 = 'x' as i32;
const KEY_CANCEL: i32 = 27;

macro_rules! log {
    ($mand:expr, $($t:tt)*) => {
        $mand.encounter.log(format!($($t)*));
    };
}

macro_rules! set_char_value {
    ($self:ident, $name:literal, $title:literal, $val:ident) => {{
        let mut char = &mut $self.get_selected_char_mut();
        let result = textbox_open($title);
        char.$val = result.parse::<i32>().unwrap_or(char.$val);
    }
    log!(
        $self,
        "{} {} set to {}. ",
        $name,
        $self.get_selected_char().name,
        $self.get_selected_char().initiative
    );
    $self.encounter.update();
    $self.save_char_list();};
}

pub struct MainWindow {
    leftwin: *mut i8,
    rightwin: *mut i8,
    logwin: *mut i8,
    selpos: i32,
    markedpos: i32,
    message: Option<String>,
    action: Option<Action>,
    encounter: Encounter,
    monsterdb: MonsterDB,
}

#[derive(Clone)]
struct Action {
    position: i32,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let midw = ncurses::COLS() / 2;
        let midh = ncurses::LINES() / 2;

        let mut window = MainWindow {
            leftwin: ncurses::subwin(ncurses::stdscr(), midh, midw, 0, 0),
            rightwin: ncurses::subwin(ncurses::stdscr(), ncurses::LINES(), midw, 0, midw),
            logwin: ncurses::subwin(ncurses::stdscr(), midh, midw, midh, 0),
            encounter: Encounter::new(),
            monsterdb: MonsterDB::load(),
            selpos: 1,
            markedpos: -1,
            message: None,
            action: None,
        };
        window.load_char_list();
        window.encounter.update();
        return window;
    }

    fn cursor_move(&mut self, amount: i32) {
        self.selpos += amount;
        if self.selpos > self.encounter.charcount() as i32 {
            self.selpos = self.encounter.charcount() as i32;
        } else if self.selpos < 1 {
            self.selpos = 1
        }
    }

    fn mark_done(&mut self) {
        self.get_selected_char_mut().done ^= true;
        log!(self, "{} finished his turn. ", self.get_selected_char().name);
        self.encounter.update();
        self.save_char_list();
    }

    fn get_char_by_index(&self, index: i32) -> &Character {
        return match self.encounter.char_at(index as usize - 1) {
            Some(x) => x,
            None => {
                panic!("Selected character out of bounds");
            }
        };
    }

    fn get_char_by_index_mut(&mut self, index: i32) -> &mut Character {
        return match self.encounter.char_at_mut(index as usize - 1) {
            Some(x) => x,
            None => {
                panic!("Selected character out of bounds");
            }
        };
    }

    fn get_selected_char(&self) -> &Character {
        return self.get_char_by_index(self.selpos);
    }

    fn get_action_source(&self, action: &Action) -> &Character {
        return self.get_char_by_index(action.position);
    }

    fn get_selected_char_mut(&mut self) -> &mut Character {
        return self.get_char_by_index_mut(self.selpos);
    }

    fn get_action_source_mut(&mut self, action: &Action) -> &mut Character {
        return self.get_char_by_index_mut(action.position);
    }

    fn new_round(&mut self) {
        match textbox_open("New Round? y/N").as_str() {
            "y" => {
                log!(self, "New round! ");
                self.encounter.new_round();
                self.save_char_list();
            }
            _ => {}
        }
    }

    fn add_char(&mut self) {
        let name = textbox_open("Name: ");
        if name == "" {
            return;
        }
        let joinbattle = textbox_open("Join Battle Dice: ");
        log!(self, "{} joined combat! ", name);
        let char = Character::new(name, joinbattle.parse::<i32>().unwrap_or(0), 7);
        self.encounter.add_char(char);
        self.save_char_list();
    }

    fn add_monster(&mut self) {
        let selmonster = textbox_select("Monster: ", &self.monsterdb.get_monster_names());
        if selmonster == "" {
            return;
        }
        let label = char::from_u32(self.encounter.count_name(selmonster.as_str()) as u32 + 65);
        match self.monsterdb.get_monster_by_name(selmonster.as_str()) {
            Some(mut x) => {
                x.label = label;
                self.encounter.add_char(x);
                log!(self, "{} {} joined combat! ", selmonster, label.unwrap_or(' '));
                self.save_char_list();
            }
            None => {}
        }
    }

    fn select_target(&mut self) {
        self.action = Some(Action {
            position: self.selpos,
        });
    }

    fn decisive_attack(&mut self, action: &Action) {
        if self.get_action_source(action).crashed() || self.get_action_source(action).dead() {
            self.message = Some(format!("Crashed/Dead character cannot decisive attack"));
            return;
        }
        let hit = textbox_open("Hit (dmg/N)?").trim().to_lowercase();
        if hit == "n" {
            self.get_action_source_mut(action).do_decisive_miss();
            log!(
                self,
                "{} misses decisive attack on {}. ",
                self.get_action_source(action).name,
                self.get_selected_char().name
            );
        } else {
            match hit.parse::<i32>() {
                Ok(x) => {
                    self.get_action_source_mut(action).do_decisive_hit();
                    self.get_selected_char_mut().take_decisive_hit(x);
                    log!(
                        self,
                        "{} hits decisive attack on {}, with {} damage!",
                        self.get_action_source(action).name,
                        self.get_selected_char().name,
                        x
                    );
                }
                Err(_) => {}
            };
        }
        self.encounter.update();
        self.save_char_list();
        self.cancel();
    }

    fn withering_attack(&mut self, action: &Action) {
        if self.get_action_source(action).dead() {
            self.message = Some(format!("Dead character cannot withering attack"));
            return;
        }
        match textbox_open("Damage (-1: miss)").parse::<i32>() {
            Ok(x) => {
                let crashed = self.get_selected_char_mut().take_withering_hit(x);
                self.get_action_source_mut(action)
                    .do_withering_hit(x, crashed);
                if x == -1 {
                    log!(
                        self,
                        "{} misses withering attack on {}. ",
                        self.get_action_source(action).name,
                        self.get_selected_char().name
                    );
                } else {
                    log!(
                        self,
                        "{} hits a withering attack on {} for {} damage. ",
                        self.get_action_source(action).name,
                        self.get_selected_char().name,
                        x
                    );
                }
            }
            Err(_) => {}
        };
        self.encounter.update();
        self.save_char_list();
        self.cancel();
    }

    fn remove_char(&mut self) {
        if self.encounter.charcount() <= 1 {
            self.message = Some(String::from("Cannot remove last character"));
            return;
        }
        let text = format!("Remove {}? y/N", self.get_selected_char().name);
        if textbox_open(text.as_str()).trim().to_lowercase().as_str() != "y" {
            return;
        }
        self.encounter.removechar(self.selpos as usize - 1);
        if self.selpos as usize > self.encounter.charcount() {
            self.selpos = self.encounter.charcount() as i32;
        }
        self.save_char_list();
    }

    fn reset(&mut self) {
        match textbox_open("Reset? y/N").as_str() {
            "y" => self.encounter.reset(),
            _ => {}
        }
    }

    fn cancel(&mut self) {
        self.action = None;
    }

    fn load_char_list(&mut self) {
        self.encounter = match std::fs::read_to_string("/tmp/__exaltedcombat") {
            Ok(x) => match serde_json::from_str(&x) {
                Ok(y) => y,
                Err(e) => {
                    panic!("Error parsing previous encounter state {}", e);
                }
            },
            Err(_) => Encounter::new(),
        };
    }

    fn save_char_list(&mut self) {
        match serde_json::to_string(&self.encounter) {
            Ok(x) => match std::fs::write("/tmp/__exaltedcombat", x) {
                Ok(_) => {}
                Err(_) => {
                    self.message = Some("Unable to save encounter to file".into());
                }
            },
            Err(_) => {
                self.message = Some("Error parsing encounter to json".into());
            }
        }
    }

    fn draw_char_list(&self) {
        ncurses::werase(self.leftwin);
        ncurses::wborder(self.leftwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawtext(self.leftwin, 0, 2, "Participants", Color::White, true, true, false, false, 32);
        let mut pos: i32 = 1;
        for char in self.encounter.char_iter() {
            let color = if self.markedpos == pos - 1 {
                Color::Blue
            } else if self.action.is_some() && self.action.as_ref().unwrap().position == pos {
                Color::Magenta
            } else if char.dead() {
                Color::Red
            } else if char.crashed() {
                Color::Yellow
            } else {
                Color::White
            };

            drawtext(
                self.leftwin,
                pos,
                2,
                format!(
                    "{:<width$}{:<4}{:<4}{:<2}{:<2}{:<6}",
                    format!("{} {}", char.name, char.label.clone().unwrap_or(' ')),
                    char.initiative,
                    char.onslaught,
                    if char.done { "D" } else { "" },
                    if char.crashed() { "C" } else { "" },
                    format!("{}/{}", char.health, char.maxhealth),
                    width = (ncurses::COLS() / 2 - 23) as usize
                )
                .as_str(),
                color,
                false,
                false,
                pos == self.selpos,
                char.done,
                ncurses::COLS() / 2,
            );
            pos += 1;
        }

        drawcolor(
            self.leftwin,
            ncurses::LINES() / 2 - 1,
            2,
            HELPSTR,
            Color::White,
            ncurses::COLS() / 2 - 4,
        );
    }

    fn draw_details(&self) {
        ncurses::werase(self.rightwin);
        ncurses::wborder(self.rightwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawtext(self.rightwin, 0, 2, "Details", Color::White, true, true, false, false, 32);

        let char = self.get_selected_char();
        drawtext(
            self.rightwin,
            1,
            2,
            &char.name,
            Color::Green,
            true,
            false,
            false,
            false,
            ncurses::COLS() / 2 - 2,
        );
        drawcolor(
            self.rightwin,
            2,
            2,
            format!("Evasion: {}", char.evasion).as_str(),
            Color::Blue,
            ncurses::COLS() / 4 - 2,
        );
        drawcolor(
            self.rightwin,
            2,
            ncurses::COLS() / 4 - 1,
            format!("Parry:    {}", char.parry).as_str(),
            Color::Blue,
            ncurses::COLS() / 4 - 2,
        );
        drawcolor(
            self.rightwin,
            3,
            2,
            format!("Soak:    {}", char.soak).as_str(),
            Color::Blue,
            ncurses::COLS() / 4 - 2,
        );
        drawcolor(
            self.rightwin,
            3,
            ncurses::COLS() / 4 - 1,
            format!("Hardness: {}", char.hardness()).as_str(),
            Color::Blue,
            ncurses::COLS() / 4 - 2,
        );

        let mut pos = 4;
        if char.attacks.is_some() {
            for attack in char.attacks.as_ref().unwrap().iter() {
                drawcolor(
                    self.rightwin,
                    pos,
                    2,
                    format!("{}: {}d -> {}", attack.name, attack.dice, attack.damage).as_str(),
                    Color::Red,
                    ncurses::COLS() / 2 - 1,
                );
                pos += 1;
            }
        }
        pos += 1;

        if char.specials.is_some() {
            for special in char.specials.as_ref().unwrap().iter() {
                if pos + 2 > ncurses::LINES() - 3 {
                    drawcolor(self.rightwin, pos, 2, "...", Color::Yellow, ncurses::COLS() / 2 - 1);
                    break;
                }
                ncurses::mvwhline(
                    self.rightwin,
                    pos,
                    1,
                    ncurses::ACS_HLINE(),
                    ncurses::COLS() / 2 - 2,
                );
                drawtext(
                    self.rightwin,
                    pos + 1,
                    2,
                    special.name.as_str(),
                    Color::Yellow,
                    true,
                    false,
                    false,
                    false,
                    ncurses::COLS() / 2 - 1,
                );
                pos += 2;
                for line in textwrap::wrap(&special.text, (ncurses::COLS() / 2 - 2) as usize) {
                    drawcolor(self.rightwin, pos, 2, &line, Color::Yellow, ncurses::COLS() / 2 - 1);
                    pos += 1;
                    if pos > ncurses::LINES() - 3 {
                        break;
                    }
                }
            }
        }
    }

    fn draw_log(&self) {
        ncurses::werase(self.logwin);
        ncurses::wborder(self.logwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawtext(self.logwin, 0, 2, "Combat Log", Color::White, true, true, false, false, 32);
        for (idx, msg) in self
            .encounter
            .log_iter()
            .skip(std::cmp::max(self.encounter.log_len() as i32 - (ncurses::LINES() / 2 - 2), 0)
                as usize)
            .enumerate()
        {
            drawcolor(
                self.logwin,
                idx as i32 + 1,
                2,
                msg.as_str(),
                Color::White,
                ncurses::COLS() / 2 - 4,
            );
        }
        if self.message.is_some() {
            drawtext(
                self.logwin,
                ncurses::LINES() / 2 - 1,
                2,
                &self.message.as_ref().unwrap().as_str(),
                Color::Blue,
                false,
                false,
                false,
                true,
                ncurses::COLS() / 2,
            );
        }
    }
}

impl Drawable for MainWindow {
    fn refresh(&self) {
        ncurses::wrefresh(self.rightwin);
        ncurses::wrefresh(self.leftwin);
        ncurses::wrefresh(self.logwin);
    }

    fn process_events(&mut self, ch: i32) {
        self.message = None;
        match ch {
            KEY_UP => self.cursor_move(-1),
            KEY_DOWN => self.cursor_move(1),
            ncurses::KEY_UP => self.cursor_move(-1),
            ncurses::KEY_DOWN => self.cursor_move(1),
            KEY_MARK_DONE => self.mark_done(),
            KEY_INITIATIVE => {
                set_char_value!(self, "initiative", "Initiative: ", initiative);
            }
            KEY_ONSLAUGHT => {
                set_char_value!(self, "onslaught", "Onslaught: ", onslaught);
            }
            KEY_HEALTH => {
                set_char_value!(self, "health", "Health: ", health);
            }
            KEY_NEW_ROUND => self.new_round(),
            KEY_ADD_CHAR => self.add_char(),
            KEY_ADD_MONSTER => self.add_monster(),
            KEY_DECISIVE_ATTACK => match self.action.clone() {
                Some(x) => self.decisive_attack(&x),
                None => self.select_target(),
            },
            KEY_WITHERING_ATTACK => match self.action.clone() {
                Some(x) => self.withering_attack(&x),
                None => self.select_target(),
            },
            KEY_REMOVE => self.remove_char(),
            KEY_RESET => self.reset(),
            KEY_CANCEL => self.cancel(),
            _ => {}
        }
    }

    fn render(&self) {
        self.draw_char_list();
        self.draw_details();
        self.draw_log();
    }
}
