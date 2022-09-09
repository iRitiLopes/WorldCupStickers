use core::str::FromStr;
use crate::cli::commands::Default;
use crate::cli::commands::Show;
use crate::cli::commands::Trade;
use crate::cli::commands::Collect;
use crate::album::team::Team;
use std::env::Args;
mod commands;
use commands::Command;

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
                let arg = &args.nth(0);
                match arg {
                    Some(team_arg) => {
                        let team = Team::from_str(team_arg);
                        Box::new(Show { team: team })
                    }
                    None => Box::new(Show { team: Err(()) }),
                }
            }
            Commands::Default => Box::new(Default {}),
        }
    }
}