use crate::album::team::Team;
use crate::album::Album;

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
    pub missing: bool,
    pub repeated: bool,
    pub info: bool,
}

impl Command for Show {
    fn execute(&self, album: &mut Album) {
        match self.team {
            Ok(team) => album.show_team(team, self.missing, self.repeated, self.info),
            Err(_) => album.show(self.missing, self.repeated, self.info),
        }
    }
}

pub struct Clean {
    pub team: Result<Team, ()>,
    pub repeated: bool,
    pub all: bool,
}

impl Command for Clean {
    fn execute(&self, album: &mut Album) {
        if !self.all && self.team.is_err() {
            println!(
                "Invalid args. Example: \n
                stickers clean BRA\n
                stickers clean all <to clear all national teams repeateds stickers"
            );
            return;
        }
        if self.all {
            album.clean_all(self.repeated);
            return;
        }
        album.clean(self.team, self.repeated);
    }
}

pub struct Nations {}

impl Command for Nations {
    fn execute(&self, album: &mut Album) {
        album.show_nations()
    }
}
