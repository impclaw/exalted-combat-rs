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
    #[serde(default = "Character::default_label")]
    pub label: Option<char>,
    #[serde(default = "Character::default_zero")]
    pub initiative: i32,
    #[serde(default = "Character::default_zero")]
    crashed_turns: i32,
    pub joinbattle: i32,
    #[serde(default = "Character::default_zero")]
    pub onslaught: i32,
    #[serde(default = "Character::default_false")]
    pub done: bool,
    #[serde(rename = "health")]
    pub maxhealth: i32,
    #[serde(default = "Character::default_zero")]
    #[serde(rename = "current_health")]
    pub health: i32,
    pub evasion: i32,
    pub parry: i32,
    pub soak: i32,
    #[serde(default = "Character::default_zero")]
    pub hardness: i32,
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
            crashed_turns: 0, 
            onslaught: 0,
            done: false,
            evasion: 0,
            parry: 0,
            soak: 0,
            hardness: 0,
            attacks: None,
            specials: None,
        }
    }
    fn default_label() -> Option<char> { None }
    fn default_zero() -> i32 { 0 }
    fn default_false() -> bool { false }
    
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
        self.initiative = roll_dice(self.joinbattle) + 3;
        if self.initiative < 0 {
            self.initiative = 0;
        }
        self.health = self.maxhealth;
    }
    pub fn finish(&mut self) {
        if self.crashed() && self.crashed_turns < 2 {
            self.crashed_turns += 1;
        } else if self.crashed() && self.crashed_turns >= 2 {
            self.initiative = 3;
            self.crashed_turns = 0;
        }
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
    pub fn take_withering_hit(&mut self, damage: i32) -> bool {
        let mut crashed = false;
        if damage >= 0 {
            crashed = self.crashed();
            self.initiative -= damage;
            self.onslaught -= 1;
            crashed = self.crashed() && !crashed;
        }
        return crashed;
    }
    pub fn do_withering_hit(&mut self, damage: i32, crashed: bool) {
        if damage < 0 {
            self.initiative += 1;
        } else {
            self.initiative += damage + 1;
            if crashed {
                self.initiative += 5;
            }
        }
        self.finish();
    }
    pub fn hardness(&self) -> i32 {
        return match self.crashed() {
            true => 0, 
            false => self.hardness
        };
    }
    pub fn take_decisive_hit(&mut self, damage: i32) {
        if damage > self.hardness() {
            self.health -= damage;
        }
    }
    pub fn do_decisive_hit(&mut self) {
        self.initiative = 3;
        self.finish();
    }
    pub fn do_decisive_miss(&mut self) {
        if self.initiative > 10 {
            self.initiative -= 3;
        } else if self.initiative > 0 {
            self.initiative -= 2;
        }
        self.finish();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encounter {
    characters: Vec<Character>,
    log: Vec<String>, 
}

impl Encounter {
    pub fn new() -> Encounter {
        Encounter {
            characters: Character::load_characters(),
            log: Vec::new(),
        }
    }
    
    pub fn log(&mut self, message: String) {
        self.log.push(message);
    }

    pub fn log_iter(&self) -> std::slice::Iter<String> {
        return self.log.iter();
    }

    pub fn log_len(&self) -> usize {
        return self.log.len();
    }

    pub fn charcount(&self) -> usize {
        return self.characters.len();
    }

    pub fn char_iter(&self) -> std::slice::Iter<Character> {
        return self.characters.iter();
    }

    pub fn char_at(&self, index: usize) -> Option<&Character> {
        return self.characters.get(index);
    }

    pub fn char_at_mut(&mut self, index: usize) -> Option<&mut Character> {
        return self.characters.get_mut(index);
    }

    pub fn new_round(&mut self) {
        for char in &mut self.characters {
            char.ready();
        }
        self.update();
    }

    pub fn add_char(&mut self, mut char: Character) {
        char.reset();
        self.characters.push(char);
        self.update();
    }

    pub fn count_name(&self, name: &str) -> usize {
        return self.characters.iter().filter(|x| x.name == name).count();
    }

    pub fn removechar(&mut self, index: usize) {
        self.characters.remove(index);
    }

    pub fn reset(&mut self) {
        self.log.clear();
        self.characters = Character::load_characters();
        self.update();
    }

    pub fn update(&mut self) {
        self.characters.sort_by_key(|c| c.sortkey());
    }
}

pub struct MonsterDB {
    monsters: Vec<Character>,
}

impl MonsterDB {
    pub fn load() -> MonsterDB {
        MonsterDB {
            monsters: Character::load_monsters(),
        }
    }

    pub fn get_monster_names(&self) -> Vec<&str> {
        return self.monsters.iter().map(|x| x.name.as_str()).collect();
    }

    pub(crate) fn get_monster_by_name(&self, name: &str) -> Option<Character> {
        for monster in self.monsters.iter() {
            if monster.name == name {
                let mut monster_copy = monster.clone();
                monster_copy.reset();
                return Some(monster_copy);
            }
        }
        return None;
    }
}
