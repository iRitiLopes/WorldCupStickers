use std::env::Args;
use std::str::FromStr;

use crate::album::Album;
use crate::team::Team;

#[path = "../album/album.rs"]
mod album;

#[derive(Debug, PartialEq)]
enum Commands {
    Collect,
    Trade,
    Show,
    Default,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trade" | "trocar" => Ok(Self::Trade),
            "collect" | "coletar" => Ok(Self::Collect),
            "show" | "mostrar" => Ok(Self::Show),
            _ => Ok(Self::Default),
        }
    }
}

pub struct Cli {}

impl Cli {
    pub fn parse(args: &mut Args) -> Box<dyn Command + 'static> {
        let command = Commands::from_str(&args.nth(1).unwrap()).unwrap();
        match command {
            Commands::Collect => {
                let team = Team::from_str(&args.nth(0).unwrap()).unwrap();
                let ids: Vec<String> = args.collect();
                Box::new(Collect {
                    team: team,
                    player_ids: ids,
                })
            }
            Commands::Trade => {
                let team = Team::from_str(&args.nth(0).unwrap()).unwrap();
                let ids: Vec<String> = args.collect();
                Box::new(Trade {
                    team: team,
                    player_ids: ids,
                })
            }
            Commands::Show => {
                let team = Team::from_str(&args.nth(0).unwrap());
                Box::new(Show { team: team })
            }
            Commands::Default => Box::new(Default {}),
        }
    }
}

pub trait Command {
    fn execute(&self, album: &mut Album);
}

pub struct Trade {
    pub team: Team,
    pub player_ids: Vec<String>,
}

impl Command for Trade {
    fn execute(&self, album: &mut Album) {
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

pub struct Show {
    pub team: Result<Team, ()>,
}

impl Command for Show {
    fn execute(&self, album: &mut Album) {
        match  self.team {
            Ok(team) => {
                let n_team = album.get_national_team(team);
                match n_team {
                    Ok(n) => println!("{}", n),
                    Err(_) => println!("{}", album),
                }
            },
            Err(_) => println!("{}", album)
        }
    }
}
