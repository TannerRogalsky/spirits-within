use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Stat {
    Knowledge,
    Discipline,
    Proficiency,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Spirit {
    TheVeil,
    Mirror,
    ThePath,
    Shadows,
    Instinct,
    Reason,
    Whispers,
    Respect,
    Drama,
    Motion,
    Muscle,
    Kinesis,
    Glamour,
    Balance,
    ThePulse,
}

impl Spirit {
    pub fn stat(self) -> Stat {
        match self {
            Spirit::TheVeil => Stat::Discipline,
            Spirit::Mirror => Stat::Discipline,
            Spirit::ThePath => Stat::Knowledge,
            Spirit::Shadows => Stat::Proficiency,
            Spirit::Instinct => Stat::Proficiency,
            Spirit::Reason => Stat::Knowledge,
            Spirit::Whispers => Stat::Knowledge,
            Spirit::Respect => Stat::Discipline,
            Spirit::Drama => Stat::Knowledge,
            Spirit::Motion => Stat::Proficiency,
            Spirit::Muscle => Stat::Discipline,
            Spirit::Kinesis => Stat::Proficiency,
            Spirit::Glamour => Stat::Proficiency,
            Spirit::Balance => Stat::Discipline,
            Spirit::ThePulse => Stat::Knowledge,
        }
    }

    pub fn description_short(self) -> &'static str {
        match self {
            Spirit::TheVeil => "Explore the abyss and know its power, consult the dead, challenge death's rule.",
            Spirit::Mirror => "See yourself clearly, see others' reflections in you, blend in.",
            Spirit::ThePath => "See how things got here, where they're going, when to change paths.",
            Spirit::Shadows => "Stay hidden, follow behind, strike unseen.",
            Spirit::Instinct => "Enter fight or flight mode, focus on your every sense, follow passion and instinct.",
            Spirit::Reason => "Use logic to choose correctly, change minds, manipulate symbols.",
            Spirit::Whispers => "Recall secrets you've heard, steal new ones from them, sow evil seeds.",
            Spirit::Respect => "Demand acknowledgement, enforce obedience, know who rules whom.",
            Spirit::Drama => "Take the stage, spot bad actors, create a fiction.",
            Spirit::Motion => "Move with purpose, move every nerve in concert, maximize speed.",
            Spirit::Muscle => "Kick things in, beat them down, roll with the punches.",
            Spirit::Kinesis => "Take control of a thing, think how it thinks, guide how it moves.",
            Spirit::Glamour => "Draw others in, distract and confuse, find the beauty.",
            Spirit::Balance => "Steady body, steady hands, steady mind.",
            Spirit::ThePulse => "Know the roots, swim with the current's flow, chase the future.",
        }
    }

    pub fn list() -> [Spirit; 15] {
        use Spirit::*;
        [
            TheVeil, Mirror, ThePath, Shadows, Instinct, Reason, Whispers, Respect, Drama, Motion,
            Muscle, Kinesis, Glamour, Balance, ThePulse,
        ]
    }
}

#[wasm_bindgen(js_name = SpiritConnection)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Connection {
    Ineptitude,
    Competence,
    Expertise,
    Mastery,
    Wizardry,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Connection::Ineptitude => write!(f, "Ineptitude"),
            Connection::Competence => write!(f, "Competence"),
            Connection::Expertise => write!(f, "Expertise"),
            Connection::Mastery => write!(f, "Mastery"),
            Connection::Wizardry => write!(f, "Wizardry"),
        }
    }
}

// #[wasm_bindgen(js_class = SpiritConnection)]
impl Connection {
    pub fn die(self) -> u8 {
        match self {
            Connection::Ineptitude => 20,
            Connection::Competence => 12,
            Connection::Expertise => 10,
            Connection::Mastery => 8,
            Connection::Wizardry => 6,
        }
    }
}

type SpiritTuple = (Spirit, Connection);

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpiritSelection {
    the_veil: SpiritTuple,
    mirror: SpiritTuple,
    the_path: SpiritTuple,
    shadows: SpiritTuple,
    instinct: SpiritTuple,
    reason: SpiritTuple,
    whispers: SpiritTuple,
    respect: SpiritTuple,
    drama: SpiritTuple,
    motion: SpiritTuple,
    muscle: SpiritTuple,
    kinesis: SpiritTuple,
    glamour: SpiritTuple,
    balance: SpiritTuple,
    the_pulse: SpiritTuple,
}

impl SpiritSelection {
    pub const MASTERY_COUNT: u32 = 1;
    pub const EXPERTISE_COUNT: u32 = 3;
    pub const COMPETENCE_COUNT: u32 = 5;
    pub const INEPTITUDE_COUNT: u32 = 6;
}

impl TryFrom<[Connection; 15]> for SpiritSelection {
    type Error = ();

    fn try_from(spirits: [Connection; 15]) -> Result<Self, Self::Error> {
        if is_valid(&spirits) {
            Ok(spirit_selection_unchecked(&spirits))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Vec<Connection>> for SpiritSelection {
    type Error = ();

    fn try_from(spirits: Vec<Connection>) -> Result<Self, Self::Error> {
        if spirits.len() == 15 && is_valid(&spirits) {
            Ok(spirit_selection_unchecked(&spirits))
        } else {
            Err(())
        }
    }
}

impl IntoIterator for &SpiritSelection {
    type Item = SpiritTuple;
    type IntoIter = std::array::IntoIter<Self::Item, 15>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.the_veil,
            self.mirror,
            self.the_path,
            self.shadows,
            self.instinct,
            self.reason,
            self.whispers,
            self.respect,
            self.drama,
            self.motion,
            self.muscle,
            self.kinesis,
            self.glamour,
            self.balance,
            self.the_pulse,
        ]
        .into_iter()
    }
}

fn spirit_selection_unchecked(spirits: &[Connection]) -> SpiritSelection {
    SpiritSelection {
        the_veil: (Spirit::TheVeil, spirits[0]),
        mirror: (Spirit::Mirror, spirits[1]),
        the_path: (Spirit::ThePath, spirits[2]),
        shadows: (Spirit::Shadows, spirits[3]),
        instinct: (Spirit::Instinct, spirits[4]),
        reason: (Spirit::Reason, spirits[5]),
        whispers: (Spirit::Whispers, spirits[6]),
        respect: (Spirit::Respect, spirits[7]),
        drama: (Spirit::Drama, spirits[8]),
        motion: (Spirit::Motion, spirits[9]),
        muscle: (Spirit::Muscle, spirits[10]),
        kinesis: (Spirit::Kinesis, spirits[11]),
        glamour: (Spirit::Glamour, spirits[12]),
        balance: (Spirit::Balance, spirits[13]),
        the_pulse: (Spirit::ThePulse, spirits[14]),
    }
}

fn is_valid(ss: &[Connection]) -> bool {
    let mut mastery_count = 0;
    let mut expertise_count = 0;
    let mut competence_count = 0;
    let mut ineptitude_count = 0;

    for connection in ss {
        match connection {
            Connection::Ineptitude => ineptitude_count += 1,
            Connection::Competence => competence_count += 1,
            Connection::Expertise => expertise_count += 1,
            Connection::Mastery => mastery_count += 1,
            Connection::Wizardry => return false,
        }
    }

    mastery_count == SpiritSelection::MASTERY_COUNT
        && expertise_count == SpiritSelection::EXPERTISE_COUNT
        && competence_count == SpiritSelection::COMPETENCE_COUNT
        && ineptitude_count == SpiritSelection::INEPTITUDE_COUNT
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Stats {
    discipline: u32,
    knowledge: u32,
    proficiency: u32,
}

#[wasm_bindgen]
impl Stats {
    pub fn new(ss: &SpiritSelection) -> Self {
        let mut discipline = 1;
        let mut knowledge = 5;
        let mut proficiency = 5;

        for (spirit, connection) in ss {
            let stat = match spirit.stat() {
                Stat::Knowledge => &mut knowledge,
                Stat::Discipline => &mut discipline,
                Stat::Proficiency => &mut proficiency,
            };

            let increment = match connection {
                Connection::Ineptitude => 0,
                Connection::Competence => 1,
                Connection::Expertise => 3,
                Connection::Mastery => 5,
                Connection::Wizardry => 0,
            };

            *stat += increment;
        }

        Self {
            discipline,
            knowledge,
            proficiency,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ss = SpiritSelection::try_from([
            Connection::Mastery,
            Connection::Expertise,
            Connection::Expertise,
            Connection::Expertise,
            Connection::Competence,
            Connection::Competence,
            Connection::Competence,
            Connection::Competence,
            Connection::Competence,
            Connection::Ineptitude,
            Connection::Ineptitude,
            Connection::Ineptitude,
            Connection::Ineptitude,
            Connection::Ineptitude,
            Connection::Ineptitude,
        ])
        .unwrap();
        let stats = Stats::new(&ss);
        assert_eq!(
            stats,
            Stats {
                discipline: 10,
                knowledge: 11,
                proficiency: 9
            }
        )
    }
}
