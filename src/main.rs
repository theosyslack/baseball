mod args;
mod events;
mod game;
mod inning;
mod player;
mod team;

use args::{Cli, Command, GenCommand, ReadCommand};
use clap::Parser;
use player::Player;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, prelude::*};
use team::Team;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Gen {
            commands,
            should_save_file,
        } => {
            match commands {
                GenCommand::Player => {
                    let player = Player::fake();

                    if *should_save_file {
                        write_json_file(&player.id, &player);
                    } else {
                        println!("{}", print(&player));
                    }
                }
                GenCommand::Team => {
                    let team = Team::fake();

                    if *should_save_file {
                        write_json_file(&team.id, &team);
                    } else {
                        println!("{}", print(&team));
                    }
                }
            };
        }
        Command::Read { commands } => {
            match commands {
                ReadCommand::Player { filename } => {
                    let string = fs::read_to_string(filename).expect("Could not find file");
                    let value: Player = serde_json::from_str(&string)
                        .expect(&format!("{filename} is not a valid Player."));

                    println!("{:?}", &value);
                }
                ReadCommand::Team { filename } => {
                    let string = fs::read_to_string(filename).expect("Could not find file");
                    let value: Team = serde_json::from_str(&string)
                        .expect(&format!("{filename} is not a valid Team."));

                    println!("{}", value);
                }
            }
            // let string = read_json_file(filename);
        }
    }
}

fn write_json_file<T: Serialize>(filename: &str, value: &T) {
    let mut file = File::create(format!("{}.json", filename)).expect("could not create file");
    let content = print(value);

    file.write_all(content.as_bytes()).expect("could not write")
}

fn print<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect("could not stringify")
}

// fn read_json_file (filename: &str) -> io::Result<String> {
//     fs::read_to_string(&format!("{}.json", filename))

//     // let value = serde_json::from_str(&content);

//     // if let Ok(value) =  value {
//     //     Ok(value)
//     // } else {
//     //     Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid"))
//     // }

// }
