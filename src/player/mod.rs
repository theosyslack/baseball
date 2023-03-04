pub mod handedness;
pub mod name;
pub mod position;

use std::u8;

use self::{handedness::PlayerHandedness, name::PlayerName, position::PlayerPosition};
use fake::uuid::UUIDv1;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, Debug, Dummy, Deserialize, Serialize)]
pub struct Player {
    #[dummy(faker = "UUIDv1")]
    pub id: String,
    pub name: PlayerName,
    handed: PlayerHandedness,
    #[dummy(faker = "0..99")]
    number: usize,
    position: Option<PlayerPosition>,
    #[dummy(faker = "100")]
    energy: u8,
    #[dummy(faker = "0..100")]
    power: u8,
}

impl Player {
    pub fn new(
        name: PlayerName,
        handed: PlayerHandedness,
        number: usize,
        position: Option<PlayerPosition>,
        power: u8,
    ) -> Self {
        let id = Self::gen_id();

        Player {
            id: id.to_string(),
            name,
            handed,
            number,
            position,
            energy: 100,
            power,
        }
    }

    fn gen_id() -> Uuid {
        let id_seed: [u8; 6] = [
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        ];

        Uuid::now_v1(&id_seed)
    }

    pub fn fake() -> Self {
        Self {
            id: Self::gen_id().to_string(),
            ..Faker.fake()
        }
    }

    pub fn name(&self) -> PlayerName {
        self.name.clone()
    }

    pub fn position(&self) -> Option<PlayerPosition> {
        self.position.clone()
    }

    pub fn fake_by_position(position: PlayerPosition) -> Self {
        let fake: Self = Faker.fake();

        fake.set_position(Some(position))
    }

    pub fn number(&self) -> usize {
        self.number.clone()
    }

    pub fn set_position(&self, position: Option<PlayerPosition>) -> Self {
        Self {
            position,
            ..self.clone()
        }
    }
}
