use rand::Rng;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attack {
    pub name: String,
    pub dice: i32,
    pub damage: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Special {
    pub name: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    #[serde(skip)]
    pub label: Option<String>,
    #[serde(skip)]
    pub initiative: i32,
    pub joinbattle: i32,
    #[serde(skip)]
    pub onslaught: i32,
    #[serde(skip)]
    pub done: bool,
    #[serde(rename = "health")]
    pub maxhealth: i32,
    #[serde(skip)]
    pub health: i32,
    pub evasion: i32,
    pub parry: i32,
    pub soak: i32,
    pub hardness: Option<i32>,
    pub attacks: Option<Vec<Attack>>,
    pub specials: Option<Vec<Special>>,
}

impl Character {
    pub fn new(name: String, joinbattle: i32, maxhealth: i32) -> Character {
        Character {
            name,
            label: None,
            maxhealth,
            health: maxhealth,
            joinbattle,
            initiative: 0,
            onslaught: 0,
            done: false,
            evasion: 0,
            parry: 0,
            soak: 0,
            hardness: Some(0),
            attacks: None,
            specials: None,
        }
    }
    pub fn load_characters() -> Vec<Character> {
        let mut char_list: Vec<Character> = serde_json::from_str(
            std::fs::read_to_string("characters.json")
                .expect("Could not open characters.json")
                .as_str(),
        )
        .expect("characters.json has invalid formatting");
        for char in char_list.iter_mut() {
            char.reset();
        }
        return char_list;
    }
    pub fn load_monsters() -> Vec<Character> {
        let monster_list: Vec<Character> = serde_json::from_str(
            std::fs::read_to_string("monsters.json")
                .expect("Could not open monsters.json")
                .as_str(),
        )
        .expect("monsters.json has invalid formatting");
        return monster_list;
    }
    pub fn reset(&mut self) {
        self.initiative = dice_roll(self.joinbattle + 3);
        self.health = self.maxhealth;
    }
    pub fn finish(&mut self) {
        self.done = true;
        self.onslaught = 0;
    }
    pub fn crashed(&self) -> bool {
        self.initiative < 0
    }
    pub fn dead(&self) -> bool {
        self.health <= 0
    }
    pub fn sortkey(&self) -> i32 {
        let mut key = -self.initiative;
        if self.health <= 0 {
            key += 5000;
        }
        if self.done {
            key += 1000;
        }
        return key;
    }
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
fn dice_roll(count: i32) -> i32 {
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
