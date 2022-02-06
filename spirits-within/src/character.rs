#![allow(unused)]

use super::{Spirit, Stats};

pub struct Character {
    name: String,
    discipline: u32,
    karma: u32,
    stress: u32,
    severances: std::collections::HashSet<Spirit>,
    stats: Stats,
}

impl Character {
    pub const MAX_KARMA: u32 = 6;

    pub fn new(name: String, stats: Stats) -> Self {
        Self {
            name,
            discipline: stats.base.discipline,
            karma: 0,
            stress: 0,
            severances: Default::default(),
            stats,
        }
    }

    pub fn add_karma(&mut self, karma: u32) -> u32 {
        self.karma = Self::MAX_KARMA.max(self.karma + karma);
        self.karma
    }

    pub fn substract_karma(&mut self, karma: u32) -> Result<u32, ()> {
        match self.karma.checked_sub(karma) {
            None => Err(()),
            Some(karma) => {
                self.karma = karma;
                Ok(self.karma)
            }
        }
    }
}
