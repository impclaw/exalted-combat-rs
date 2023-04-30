use crate::util::roll_dice;
use serde::{Deserialize, Serialize};

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
        self.initiative = roll_dice(self.joinbattle + 3);
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
    pub fn ready(&mut self) {
        self.done = false;
    }
}

