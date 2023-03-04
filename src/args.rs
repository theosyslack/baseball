use std::default;

use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
   /// Name of the person to greet
   #[command(subcommand)]
   pub command: Command,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

#[derive(Debug, Serialize, Deserialize, Subcommand)]
pub enum Command {
    Gen {
        #[arg(long = "save", default_value = "false")]
        should_save_file: bool,

        #[command(subcommand)]
        commands: GenCommand,
    },

    Read {
        #[command(subcommand)]
        commands: ReadCommand
    }
}

#[derive(Debug, Serialize, Deserialize, Subcommand)]
pub enum GenCommand {
    Player,
    Team
}

#[derive(Debug, Serialize, Deserialize, Subcommand)]
pub enum ReadCommand {
    Player {
        filename: String
    },
    Team {
        filename: String
    }
}

