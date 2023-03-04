use std::fmt::Display;

use crate::player::{position::PlayerPosition, Player};
use fake::uuid::UUIDv1;
use fake::{
    faker::{address::en::CityName, company::en::BsNoun, company::en::CompanyName},
    Dummy, Fake, Faker,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, Debug, Dummy, Deserialize, Serialize)]
pub struct Team {
    #[dummy(faker = "UUIDv1")]
    pub id: String,
    #[dummy(faker = "BsNoun()")]
    pub name: String,
    mascot: Option<String>,
    #[dummy(faker = "CompanyName()")]
    stadium: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "(Faker, 26)")]
    roster: Vec<Player>,
    lineup: [String; 9],
}

impl Team {
    pub fn new(name: &str, roster: Vec<Player>) -> Self {
        let id = Self::gen_id();

        let mut team = Self {
            id: id.to_string(),
            name: name.into(),
            roster,
            ..Default::default()
        };

        team.init_lineup();
        team
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

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn roster(&self) -> Vec<Player> {
        self.roster.clone()
    }

    pub fn find_player_by_position(&self, position: &PlayerPosition) -> Option<Player> {
        self.roster
            .iter()
            .find(|p| p.position() == Some(position.clone()))
            .cloned()
    }

    pub fn find_player_by_id(&self, id: &str) -> Option<Player> {
        self.roster.iter().find(|p| p.id == id).cloned()
    }

    fn init_lineup(&mut self) {
        let position_order: [PlayerPosition; 9] = [
            PlayerPosition::DesignatedHitter,
            PlayerPosition::Catcher,
            PlayerPosition::FirstBase,
            PlayerPosition::SecondBase,
            PlayerPosition::Shortstop,
            PlayerPosition::ThirdBase,
            PlayerPosition::RightField,
            PlayerPosition::CenterField,
            PlayerPosition::LeftField,
        ];

        let lineup: Vec<Player> = position_order
            .iter()
            .map(|position| {
                self.find_player_by_position(position)
                    .unwrap_or_else(|| panic!("No {} found", position))
            })
            .collect();

        self.set_lineup([
            lineup[0].clone(),
            lineup[1].clone(),
            lineup[2].clone(),
            lineup[3].clone(),
            lineup[4].clone(),
            lineup[5].clone(),
            lineup[6].clone(),
            lineup[7].clone(),
            lineup[8].clone(),
        ]);
    }

    pub fn set_lineup(&self, lineup: [Player; 9]) -> Self {
        let self_copy = self.clone();
        Self {
            lineup: lineup.map(|p| p.id),
            ..self_copy
        }
    }

    pub fn fake() -> Self {
        let team: Team = Faker.fake();

        let original_roster = team.roster();
        let dh = original_roster
            .get(0)
            .unwrap()
            .set_position(Some(PlayerPosition::DesignatedHitter));
        let p = original_roster
            .get(1)
            .unwrap()
            .set_position(Some(PlayerPosition::Pitcher));
        let c = original_roster
            .get(2)
            .unwrap()
            .set_position(Some(PlayerPosition::Catcher));
        let b1 = original_roster
            .get(3)
            .unwrap()
            .set_position(Some(PlayerPosition::FirstBase));
        let b2 = original_roster
            .get(4)
            .unwrap()
            .set_position(Some(PlayerPosition::SecondBase));
        let ss = original_roster
            .get(5)
            .unwrap()
            .set_position(Some(PlayerPosition::Shortstop));
        let b3 = original_roster
            .get(6)
            .unwrap()
            .set_position(Some(PlayerPosition::ThirdBase));
        let rf = original_roster
            .get(7)
            .unwrap()
            .set_position(Some(PlayerPosition::RightField));
        let cf = original_roster
            .get(8)
            .unwrap()
            .set_position(Some(PlayerPosition::CenterField));
        let lf = original_roster
            .get(9)
            .unwrap()
            .set_position(Some(PlayerPosition::LeftField));

        let assigned_players = [dh, c, b1, b2, ss, b3, rf, cf, lf];

        let updated_roster: Vec<Player> =
            [[p].to_vec(), assigned_players.to_vec(), original_roster.to_vec()].concat();
        let lineup = assigned_players.map(|p| p.id);

        Self {
            id: Self::gen_id().to_string(),
            lineup,
            roster: updated_roster,
            ..team
        }
    }

    pub fn lineup_string(&self) -> String {
        let mut string = String::new();
        let iter = self.lineup.clone().into_iter().enumerate();

        for (i, player_id) in iter {
            let player = self.find_player_by_id(&player_id).unwrap();
            let order = i + 1;
            let full_name = player.name().full();
            let player_number = player.number();
            let position = player
                .position()
                .map(|p| p.code())
                .unwrap_or("Bench".to_string());
            string += &format!(
                "{}: {}, #{}, ({})\n",
                order, full_name, player_number, position
            )
        }

        string
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let line = |string: &mut String, str: &str| {
            string.push_str(&format!("{}\n", str));
        };

        line(&mut string, &format!("{} {}", self.city, self.name));
        line(&mut string, "---");

        for player in &self.roster {
            line(
                &mut string,
                &format!(
                    "  - {} ({})",
                    player.name.full(),
                    player
                        .position()
                        .map_or("Benched".to_string(), |pos| pos.code())
                ),
            );
        }

        write!(f, "{}", string)
    }
}
