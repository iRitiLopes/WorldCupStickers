use std::env::Args;
use std::str::FromStr;

use crate::album::Album;
use crate::teams::{NationalTeam, Team};

#[path = "../album/album.rs"]
mod album;

#[derive(Debug, PartialEq)]
enum Commands {
    Collect,
    Trade,
    Default,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trade" => Ok(Self::Trade),
            "collect" => Ok(Self::Collect),
            _ => Ok(Self::Default),
        }
    }
}

pub trait Command {
    fn execute(&self, album: &mut Album);
}

pub struct Cli {}

impl Cli {
    pub fn parse(args: &mut Args) -> Box<dyn Command + 'static> {
        println!("{:?}", args);
        let command = Commands::from_str(&args.nth(1).unwrap()).unwrap();
        let team = Team::from_str(&args.nth(0).unwrap()).unwrap();
        let ids: Vec<String> = args.collect();
        println!("{:?}", command);
        match command {
            Commands::Collect => Box::new(Collect {
                team: team,
                player_ids: ids,
            }),
            Commands::Trade => Box::new(Trade {
                team: team,
                player_ids: ids,
            }),
            Commands::Default => Box::new(Default {}),
        }
    }
}

pub struct Trade {
    pub team: Team,
    pub player_ids: Vec<String>,
}

impl Command for Trade {
    fn execute(&self, album: &mut Album) {
        let x = &self.player_ids[1..];
        for id in &self.player_ids {
            album.trade(self.team, id)
        }
    }
}

pub struct Collect {
    pub team: Team,
    pub player_ids: Vec<String>,
}

impl Command for Collect {
    fn execute(&self, album: &mut Album) {
        for id in &self.player_ids {
            album.collect(self.team, id)
        }
    }
}

pub struct Default {}

impl Command for Default {
    fn execute(&self, _album: &mut Album) {
        println!("Comando não conhecido!")
    }
}
