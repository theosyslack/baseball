use crate::team::Team;
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Dummy, Deserialize, Serialize)]
pub struct Game {
    #[dummy(faker = "1000..")]
    id: usize,
    teams: [Team; 2],
}

impl Game {
    pub fn new(home: Team, away: Team) -> Self {
        Self {
            teams: [home, away],
            ..Default::default()
        }
    }

    pub fn teams(&self) -> [Team; 2] {
        self.teams.clone()
    }

    pub fn fake() -> Self {
        Self {
            teams: [Team::fake(), Team::fake()],
            ..Default::default()
        }
    }
}
