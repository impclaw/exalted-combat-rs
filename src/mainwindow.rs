use crate::combat::Character;
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

pub struct MainWindow {
    leftwin: *mut i8,
    rightwin: *mut i8,
    logwin: *mut i8,
    selpos: i32,
    markedpos: i32,
    message: Option<String>,
    action: Option<Action>,
    characters: Vec<Character>,
    monsters: Vec<Character>,
}

enum ActionType {
    Withering,
    Decisive,
}

struct Action {
    position: i32,
    actiontype: ActionType,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let midw = ncurses::COLS() / 2;
        let midh = ncurses::LINES() / 2;

        let mut window = MainWindow {
            leftwin: ncurses::subwin(ncurses::stdscr(), midh, midw, 0, 0),
            rightwin: ncurses::subwin(ncurses::stdscr(), ncurses::LINES(), midw, 0, midw),
            logwin: ncurses::subwin(ncurses::stdscr(), midh, midw, midh, 0),
            characters: Character::load_characters(),
            monsters: Character::load_monsters(),
            selpos: 1,
            markedpos: -1,
            message: None,
            action: None,
        };
        window.update();
        return window;
    }

    fn cursor_move(&mut self, amount: i32) {
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

    fn get_selected_char_mut(&mut self) -> &mut Character {
        return &mut self.characters[self.selpos as usize - 1];
    }

    fn set_char_initiative(&mut self) {
        let mut char = &mut self.get_selected_char_mut();
        let result = crate::textbox::textbox_open("Initiative: ");
        char.initiative = result.parse::<i32>().unwrap_or(char.initiative);
        self.update()
    }

    fn set_char_onslaught(&mut self) {
        let mut char = &mut self.characters[self.selpos as usize - 1];
        let result = crate::textbox::textbox_open("Onslaught: ");
        char.onslaught = result.parse::<i32>().unwrap_or(char.onslaught);
        self.update();
    }

    fn set_char_health(&mut self) {
        let mut char = &mut self.characters[self.selpos as usize - 1];
        let result = crate::textbox::textbox_open("Health: ");
        char.health = result.parse::<i32>().unwrap_or(char.health);
        self.update();
    }

    fn new_round(&mut self) {
        for char in &mut self.characters {
            char.ready();
        }
        self.update();
    }

    fn add_char(&mut self) {
        let name = crate::textbox::textbox_open("Name: ");
        if name == "" {
            return;
        }
        let joinbattle = crate::textbox::textbox_open("Join Battle Dice: ");
        let char = Character::new(name, joinbattle.parse::<i32>().unwrap_or(0), 7);
        self.characters.push(char);
    }

    fn add_monster(&mut self) {
        let list: Vec<String> = self.monsters.iter().map(|x| x.name.clone()).collect();
        let selmonster = crate::textbox::textbox_select("Monster: ", &list);
        let count = self
            .characters
            .iter()
            .filter(|x| x.name == selmonster)
            .count();
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

    fn decisive_attack(&mut self) {
        if self.action.is_none() {
            self.action = Some(Action {
                position: self.selpos,
                actiontype: ActionType::Decisive,
            });
        } else {
            let action = match &self.action {
                Some(x) => x,
                None => {
                    return;
                }
            };
            if !matches!(action.actiontype, ActionType::Decisive) {
                return;
            }
            let hit = crate::textbox::textbox_open("Hit (dmg/N)?")
                .trim()
                .to_lowercase();
            if hit == "n" {
                let source = &mut self.characters[action.position as usize - 1];
                if source.initiative > 10 {
                    source.initiative -= 3;
                } else {
                    source.initiative -= 2;
                }
                source.finish();
            } else {
                let damage = match hit.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => {
                        return;
                    }
                };
                {
                    let source = &mut self.characters[action.position as usize - 1];
                    source.initiative = 3;
                    source.finish();
                }
                let target = &mut self.characters[self.selpos as usize - 1];
                target.health -= damage;
            }
            self.action = None;
            self.update();
        }
    }

    fn withering_attack(&mut self) {
        if self.action.is_none() {
            self.action = Some(Action {
                position: self.selpos,
                actiontype: ActionType::Withering,
            });
        } else {
            let action = match &self.action {
                Some(x) => x,
                None => {
                    return;
                }
            };
            if !matches!(action.actiontype, ActionType::Withering) {
                return;
            }
            let damage = match crate::textbox::textbox_open("Damage (-1: miss)").parse::<i32>() {
                Ok(x) => x,
                Err(_) => {
                    return;
                }
            };
            let mut crashed = false;
            {
                let target = &mut self.characters[self.selpos as usize - 1];
                if damage >= 0 {
                    crashed = target.crashed();
                    target.initiative -= damage;
                    target.onslaught -= 1;
                    crashed = target.crashed() && !crashed;
                }
            }
            {
                let source = &mut self.characters[action.position as usize - 1];
                if damage < 0 {
                    source.initiative += 1;
                } else {
                    source.initiative += damage;
                    if crashed {
                        source.initiative += 5;
                    }
                }
                source.finish();
            }
            self.action = None;
            self.update();
        }
    }

    fn remove_char(&mut self) {
        if self.characters.len() <= 1 {
            self.message = Some(String::from("Cannot remove last character"));
            return;
        }
        self.characters.remove(self.selpos as usize - 1);
        if self.selpos as usize > self.characters.len() {
            self.selpos = self.characters.len() as i32;
        }
    }

    fn reset(&mut self) {
        self.characters = Character::load_characters();
        self.update();
    }

    fn cancel(&mut self) {
        self.action = None;
    }

    fn update(&mut self) {
        self.characters.sort_by_key(|c| c.sortkey());
    }

    fn draw_char_list(&self) {
        ncurses::werase(self.leftwin);
        ncurses::wborder(self.leftwin, 32, 32, 0, 32, 0, 0, 0, 0);
        drawtext(self.leftwin, 0, 2, "Participants", Color::White, true, true, false, false, 32);
        let mut pos: i32 = 1;
        for char in self.characters.iter() {
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
                    format!("{} {}", char.name, char.label.clone().unwrap_or(String::from(""))),
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

        let char = match self.characters.get(self.selpos as usize - 1) {
            Some(x) => x,
            None => {
                return;
            }
        };

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
            format!("Hardness: {}", char.hardness.unwrap_or(0)).as_str(),
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
            KEY_INITIATIVE => self.set_char_initiative(),
            KEY_ONSLAUGHT => self.set_char_onslaught(),
            KEY_HEALTH => self.set_char_health(),
            KEY_NEW_ROUND => self.new_round(),
            KEY_ADD_CHAR => self.add_char(),
            KEY_ADD_MONSTER => self.add_monster(),
            KEY_DECISIVE_ATTACK => self.decisive_attack(),
            KEY_WITHERING_ATTACK => self.withering_attack(),
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
