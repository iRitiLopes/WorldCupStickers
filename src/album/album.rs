use crate::teams::{NationalTeam, Team};

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
}

impl std::fmt::Display for Album<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for n in &self.teams {
            write!(f, "{}\n", n);
        }
        write!(f, "")
    }
}
