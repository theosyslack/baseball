use fake::{Fake, Dummy, faker::{name::en::FirstName, name::en::LastName}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Dummy, Deserialize, Serialize)]
pub struct PlayerName {
    #[dummy(faker = "FirstName()")]
    first: String,
    #[dummy(faker = "LastName()")]
    last: String,
}

impl PlayerName {
    pub fn new(first: &str, last: &str) -> Self {
        Self {
            first: first.to_string(),
            last: last.to_string()
        }
    }

    pub fn first(&self) -> String {
        self.first.to_string()
    }
    pub fn last(&self) -> String {
        self.last.to_string()
    }
    pub fn full(&self) -> String {
        format!("{} {}", self.first(), self.last())
    }
}
