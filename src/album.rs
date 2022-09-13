pub mod team;
use std::{fs::{self}, error::Error};

use serde::{Deserialize, Serialize};
use team::Team;

mod national_team;
use national_team::NationalTeam;

mod sticker;

#[derive(Serialize, Deserialize)]
pub struct Album<'a> {
    #[serde(borrow)]
    teams: Vec<NationalTeam<'a>>,
}

impl<'a> Album<'_> {
    pub fn new() -> Album<'static> {
        Album {
            teams: vec![
                NationalTeam::qatar(),
                NationalTeam::ecuador(),
                NationalTeam::senegal(),
                NationalTeam::netherlands(),
                NationalTeam::england(),
                NationalTeam::usa(),
                NationalTeam::iran(),
                NationalTeam::wales(),
                NationalTeam::brazil(),
            ],
        }
    }

    pub fn collect(&mut self, team: Team, id: &'a str) {
        for t in &mut self.teams {
            if t.is(&team) {
                t.collect(id)
            }
        }
        self.store();
    }

    pub fn trade(&mut self, team: Team, id: &'a str) {
        for t in &mut self.teams {
            if t.is(&team) {
                t.trade(id);
            }
        }
        self.store()
    }

    pub fn get_national_team(&self, team: Team) -> Result<&NationalTeam, ()> {
        for t in &self.teams {
            if t.is(&team) {
                return Ok(t);
            }
        }
        Err(())
    }

    fn store(&self) {
        let ser = serde_json::to_string(self).unwrap();
        fs::write(Self::path(), ser).expect("Unable to write file!");
    }

    pub fn load(data: &'a str) -> Result<Album<'a>, Box<dyn Error>> {

        // Read the JSON contents of the file as an instance of `User`.
        let u: Album = serde_json::from_str(&data)?;

        // Return the `User`.
        Ok(u)
    }

    pub fn path() -> String {
        let dir = dirs::home_dir().expect("a");
        format!("{}/.stickers.json", dir.to_str().unwrap()).to_owned()
    }
}

impl std::fmt::Display for Album<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut message = "".to_owned();
        for n in &self.teams {
            message.push_str(&format!("{}\n", n));
        }
        write!(f, "{}", message)
    }
}