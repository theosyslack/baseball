use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Dummy, Deserialize, Serialize)]
pub enum PlayerHandedness {
    Left,
    Right,
    Ambidextrous,
}

impl Default for PlayerHandedness {
    fn default() -> Self {
        PlayerHandedness::Right
    }
}
