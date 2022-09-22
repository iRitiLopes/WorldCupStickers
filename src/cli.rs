use crate::album::team::Team;
use crate::cli::commands::Clean;
use crate::cli::commands::Collect;

use crate::cli::commands::Trade;
use core::str::FromStr;

mod commands;
use crate::album::Album;
use clap::{Parser, Subcommand};
use commands::Command;

use self::commands::Nations;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute(album: &mut Album) {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Show {
                national_team,
                repeated,
                missing,
                info
            } => match national_team {
                Some(nation) => {
                    let n = Team::from_str(nation);
                    match n {
                        Ok(n_team) => album.show_team(n_team, *missing, *repeated, *info),
                        Err(_) => album.show(*missing, *repeated, *info),
                    }
                }
                None => album.show(*missing, *repeated, *info),
            },
            Commands::Collect { national_team, ids } => {
                let n = Team::from_str(&national_team).unwrap();
                let c = Collect {
                    team: n,
                    player_ids: ids.to_vec(),
                };
                c.execute(album);
            }
            Commands::Trade { national_team, ids } => {
                let n = Team::from_str(&national_team).unwrap();
                let t = Trade {
                    team: n,
                    player_ids: ids.to_vec(),
                };
                t.execute(album);
            }
            Commands::Clean { national_team, all } => match national_team {
                Some(nation) => {
                    let n = Team::from_str(nation);
                    let c = Clean {
                        team: n,
                        repeated: true,
                        all: *all,
                    };
                    c.execute(album);
                }
                None => {
                    let c = Clean {
                        team: Err(()),
                        repeated: true,
                        all: *all,
                    };
                    c.execute(album)
                }
            },
            Commands::Nations {} => {
                let n = Nations {};
                n.execute(album)
            }
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Show stickers
    Show {
        #[clap(short, long, action)]
        national_team: Option<String>,

        #[clap(short, long, action, default_value_t = false)]
        repeated: bool,

        #[clap(short, long, action, default_value_t = false)]
        missing: bool,

        ///Show complete info about your stickers
        #[clap(short, long, action, default_value_t = false)]
        info: bool
    },

    /// Collect Stickers
    Collect {
        #[clap(short, long, action)]
        national_team: String,

        #[clap(short, long, action, value_parser, use_value_delimiter = true, value_delimiter=',')]
        ids: Vec<String>,
    },

    /// Trade Stickers
    Trade {
        #[clap(short, long, action)]
        national_team: String,

        #[clap(short, long, action, value_parser, use_value_delimiter = true, value_delimiter=',')]
        ids: Vec<String>,
    },

    /// Clean Stickers
    Clean {
        /// Obrigatory if all is false
        #[clap(short, long, action)]
        national_team: Option<String>,

        ///Clean all the repeated for all nations
        #[clap(short, long, action, default_value_t = false)]
        all: bool,
    },

    Nations {},
}
