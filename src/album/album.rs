use crate::national_team::{NationalTeam};
use crate::team::Team;

pub struct Album<'a> {
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
    }

    pub fn trade(&mut self, team: Team, id: &'a str) {
        for t in &mut self.teams {
            if t.is(&team) {
                t.trade(id);
            }
        }
    }

    pub fn get_national_team(&self, team: Team) -> Result<&NationalTeam, ()> {
        for t in &self.teams {
            if t.is(&team) {
                return Ok(t)
            }
        }
        Err(())
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
