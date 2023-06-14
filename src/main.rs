use std::fmt::{Display, Formatter};

use crate::Character::{CapeMario, FireMario, SuperMario, Mario};
use crate::Food::{Fire, MushRoom, Feather};

#[derive(Debug, Clone, PartialEq)]
enum Character {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

impl Character {
    fn eat(&self, food: Food) -> Character {
        match (self, food) {
            (Mario, MushRoom) => SuperMario,
            (Mario | SuperMario | FireMario | CapeMario, Fire) => FireMario,
            (Mario | SuperMario | FireMario | CapeMario, Feather) => CapeMario,
            (_, MushRoom) => self.clone(),
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mario => { write!(f, "Mario") }
            SuperMario => { write!(f, "SuperMario") }
            FireMario => { write!(f, "FireMario") }
            CapeMario => { write!(f, "CapeMario") }
        }
    }
}

enum Food {
    MushRoom,
    Fire,
    Feather,
}

struct World {
    hero: Character,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hero)
    }
}

impl World {
    fn eat(&mut self, food: Food) {
        self.hero = self.hero.eat(food);
    }
}

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let a = SuperMario;

    let result = a.eat(MushRoom);

    let mut world = World { hero: a };

    println!("hero of this world is {}", world);
    world.eat(Fire);
    println!("hero of this world is {}", world);
    println!("{:?}", result);

    println!("toto");

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_transform_mario_with_mushroom() {
        // Given / When / Then
        assert!(matches!(Mario.eat(MushRoom), SuperMario));
        assert!(matches!(SuperMario.eat(MushRoom), SuperMario));
        assert!(matches!(FireMario.eat(MushRoom), FireMario));
        assert!(matches!(CapeMario.eat(MushRoom), CapeMario));
    }
    #[test]
    fn should_transform_mario_with_fire() {
        // Given / When / Then
        assert!(matches!(Mario.eat(Fire), FireMario));
        assert!(matches!(SuperMario.eat(Fire), FireMario));
        assert!(matches!(FireMario.eat(Fire), FireMario));
        assert!(matches!(CapeMario.eat(Fire), FireMario));
    }
    #[test]
    fn should_transform_mario_with_feather() {
        // Given / When / Then
        assert!(matches!(Mario.eat(Feather), CapeMario));
        assert!(matches!(SuperMario.eat(Feather), CapeMario));
        assert!(matches!(FireMario.eat(Feather), CapeMario));
        assert!(matches!(CapeMario.eat(Feather), CapeMario));
    }
}