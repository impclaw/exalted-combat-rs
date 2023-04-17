
#[derive(Clone, Copy)]
pub enum Color {
    White = 1, 
    Red = 2, 
    Blue = 3, 
    Green = 4, 
}

pub trait Drawable {
    fn refresh(&self);
    fn render(&self);
}

pub struct Character {
    pub name: String,
    joinbattle: i32, 
    pub initiative: i32, 
    pub onslaught: i32, 
    done: bool, 
    maxhealth: i32, 
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
}

