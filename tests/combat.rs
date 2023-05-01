#[allow(unused_imports)]
use exalted_combat::combat::*;

#[test]
fn alive_and_dead_test() {
    let mut char = Character::new(String::from("Test"), 0, 1);
    assert!(!char.dead());
    char.health = 0;
    assert!(char.dead());
}

#[test]
fn initiative_crash_correlation() {
    let mut char = Character::new(String::from("Test"), 0, 1);
    char.initiative = 1;
    assert!(!char.crashed());
    char.initiative = 0;
    assert!(!char.crashed());
    char.initiative = -1;
    assert!(char.crashed());
}

#[test]
fn withering_attack_initiative() {
    let mut attacker = Character::new(String::from("Test"), 0, 1);
    let mut defender = Character::new(String::from("Test"), 0, 1);
    attacker.initiative = 1;
    defender.initiative = 5;

    //An attack with 0 damage, only +1i for attacker
    attacker.do_withering_hit(0, defender.take_withering_hit(0));
    assert_eq!(defender.initiative, 5);
    assert_eq!(attacker.initiative, 1 + 1);

    //An attack with 3 damage, +4i for attacker, -3i for defender
    attacker.do_withering_hit(3, defender.take_withering_hit(3));
    assert_eq!(defender.initiative, 2);
    assert_eq!(attacker.initiative, 2 + 1 + 3);
}

#[test]
fn withering_attack_crash_initiative() {
    let mut attacker = Character::new(String::from("Test"), 0, 1);
    let mut defender = Character::new(String::from("Test"), 0, 1);
    attacker.initiative = 1;
    defender.initiative = 1;

    attacker.do_withering_hit(2, defender.take_withering_hit(2));
    assert_eq!(defender.initiative, -1);
    assert_eq!(attacker.initiative, 1 + 1 + 2 + 5);
}

#[test]
fn decisive_attack_miss() {
    let mut attacker = Character::new(String::from("Test"), 0, 1);
    attacker.initiative = 10;
    attacker.do_decisive_miss();
    assert_eq!(attacker.initiative, 8);

    attacker.initiative = 11;
    attacker.do_decisive_miss();
    assert_eq!(attacker.initiative, 8);
}

#[test]
fn decisive_attack_hit() {
    let mut attacker = Character::new(String::from("Test"), 0, 5);
    let mut defender = Character::new(String::from("Test"), 0, 5);
    attacker.initiative = 5;
    defender.initiative = 5;

    attacker.do_decisive_hit();
    defender.take_decisive_hit(3);
    assert_eq!(attacker.initiative, 3);
    assert_eq!(defender.health, 2);

    attacker.initiative = 8;
    attacker.do_decisive_hit();
    defender.take_decisive_hit(0);
    assert_eq!(attacker.initiative, 3);
    assert_eq!(defender.health, 2);
}

#[test]
fn decisive_hit_hardness() {
    let mut defender = Character::new(String::from("Test"), 0, 5);

    defender.initiative = 5;
    defender.hardness = Some(5);
    defender.take_decisive_hit(3);
    assert_eq!(defender.health, 5);
    
    defender.take_decisive_hit(5);
    assert_eq!(defender.health, 5);

    defender.take_decisive_hit(6);
    assert_eq!(defender.health, -1);

    //Test if crashed characters have no hardness
    defender.initiative = -1;
    defender.health = 5;
    defender.take_decisive_hit(3);
    assert_eq!(defender.health, 2);
}
