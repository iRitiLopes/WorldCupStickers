use crate::album::team::Team;
use crate::cli::commands::Collect;
use crate::cli::commands::Default;
use crate::cli::commands::Show;
use crate::cli::commands::Trade;
use core::str::FromStr;
use std::env::Args;
mod commands;
use commands::Command;

#[derive(Debug, PartialEq)]
enum Commands {
    Collect,
    Trade,
    Show,
    Export,
    Default,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trade" | "trocar" => Ok(Self::Trade),
            "collect" | "coletar" => Ok(Self::Collect),
            "show" | "mostrar" => Ok(Self::Show),
            "export" | "exportar" => Ok(Self::Export),
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
                let mut missing = false;
                let mut repeated = false;

                match args.nth(0) {
                    Some(option) => {
                        if option.eq(&String::from("--missing")) {
                            missing = true;
                        } else if option.eq(&String::from("--repeated")) {
                            repeated = true;
                        } else {
                            missing = false;
                            repeated = false;
                        }
                    }
                    None => {
                        missing = false;
                        repeated = false;
                    }
                };
                match arg {
                    Some(team_arg) => {
                        let team = Team::from_str(team_arg);
                        Box::new(Show {
                            team: team,
                            missing: missing,
                            repeated: repeated,
                        })
                    }
                    None => Box::new(Show {
                        team: Err(()),
                        missing: missing,
                        repeated: repeated,
                    }),
                }
            }
            Commands::Export => Box::new(Default {}),
            Commands::Default => Box::new(Default {}),
        }
    }
}
