use std::fmt::Display;

use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Dummy, PartialEq, Deserialize, Serialize)]
pub enum PlayerPosition {
    DesignatedHitter,
    Pitcher,
    Catcher,
    FirstBase,
    SecondBase,
    Shortstop,
    ThirdBase,
    RightField,
    CenterField,
    LeftField,
}

impl Display for PlayerPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerPosition::DesignatedHitter => write!(f, "Designated Hitter"),
            PlayerPosition::Pitcher => write!(f, "Pitcher"),
            PlayerPosition::Catcher => write!(f, "Catcher"),
            PlayerPosition::FirstBase => write!(f, "First Base"),
            PlayerPosition::SecondBase => write!(f, "Second Base"),
            PlayerPosition::Shortstop => write!(f, "Shortstop"),
            PlayerPosition::ThirdBase => write!(f, "Third Base"),
            PlayerPosition::RightField => write!(f, "Right Field"),
            PlayerPosition::CenterField => write!(f, "Center Field"),
            PlayerPosition::LeftField => write!(f, "Left Field"),
        }
    }
}

impl PlayerPosition {
    pub fn code(&self) -> String {
        let str = match self {
            PlayerPosition::DesignatedHitter => "DH",
            PlayerPosition::Pitcher => "P",
            PlayerPosition::Catcher => "C",
            PlayerPosition::FirstBase => "1B",
            PlayerPosition::SecondBase => "2B",
            PlayerPosition::Shortstop => "SS",
            PlayerPosition::ThirdBase => "3B",
            PlayerPosition::RightField => "RF",
            PlayerPosition::CenterField => "CF",
            PlayerPosition::LeftField => "LF",
        };

        str.to_string()
    }
}
