#![allow(unused)]

use super::{character::Character, Spirit};

pub struct Game {
    characters: Vec<GameCharacter>,
    state: GameState,
}

impl Game {
    pub fn new<C>(characters: C) -> Self
    where
        C: IntoIterator<Item = Character>,
    {
        Self {
            characters: characters
                .into_iter()
                .map(|base| GameCharacter { base, mood: None })
                .collect(),
            state: GameState::Downtime,
        }
    }
}

struct GameCharacter {
    base: Character,
    mood: Option<Spirit>,
}

enum GameState {
    Mission,
    Downtime,
}
