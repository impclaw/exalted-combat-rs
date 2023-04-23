
#[derive(Clone, Copy)]
pub enum Color {
    White = 1, 
    Red = 2, 
    Blue = 3, 
    Green = 4, 
    Yellow = 5, 
}

pub trait Drawable {
    fn refresh(&self);
    fn render(&self);
    fn process_events(&mut self, ch:i32);
}

pub struct Character {
    pub name: String,
    joinbattle: i32, 
    pub initiative: i32, 
    pub onslaught: i32, 
    pub done: bool, 
    pub maxhealth: i32, 
    pub health: i32, 
    evasion: i32, 
    parry: i32, 
    soak: i32, 
    hardness: i32, 
}

impl Character {
    pub fn new(name:String, joinbattle:i32, maxhealth:i32) -> Character { 
        Character {
            name,
            joinbattle, 
            maxhealth,
            health: maxhealth,
            initiative: 0, 
            onslaught: 0,
            done: false,
            evasion: 0,
            parry: 0,
            soak: 0,
            hardness: 0,
        }
    }
    pub fn defaults() -> Vec<Character> {
        vec![
            Character::new("Oswald".into(), 4, 12),
            Character::new("Embla".into(), 5, 10), 
        ]
    }
    pub fn crashed(&self) -> bool {
        self.initiative <= 0
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

