use crate::sticker::Sticker;
use crate::team::Team;
use std::{collections::HashMap, vec};

pub struct NationalTeam<'a> {
    team: Team,
    stickers: HashMap<&'a str, Sticker<'a>>,
}

impl std::fmt::Display for NationalTeam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut message = format!("{}\n", self.team).to_owned();
        for (_sticker_id, sticker) in &self.stickers {
            message.push_str(&format!("\t- {}\n", sticker));
        }
        write!(f, "{}", message)
    }
}

impl NationalTeam<'_> {
    fn new(team: Team, stickers: Vec<Sticker>) -> NationalTeam {
        let mut hm: HashMap<&str, Sticker> = HashMap::new();

        for s in stickers {
            hm.insert(s.id, s);
        }
        NationalTeam {
            team: team,
            stickers: hm,
        }
    }

    pub fn is(&self, team: &Team) -> bool {
        return self.team == *team;
    }

    pub fn brazil() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Allison"),
            Sticker::new("4", "Ederson"),
            Sticker::new("5", "Alex Sandro"),
            Sticker::new("6", "Danilo"),
            Sticker::new("7", "Eder Militão"),
            Sticker::new("8", "Marquinhos"),
            Sticker::new("9", "Thiago Silva"),
            Sticker::new("10", "Casemiro"),
            Sticker::new("11", "Philippe Coutinho"),
            Sticker::new("12", "Fabinho"),
            Sticker::new("13", "Fred"),
            Sticker::new("14", "Lucas Paquetá"),
            Sticker::new("15", "Eder Antony"),
            Sticker::new("16", "Gabriel Jesus"),
            Sticker::new("17", "Neymar Jr"),
            Sticker::new("18", "Raphinha"),
            Sticker::new("19", "Richarlison"),
            Sticker::new("20", "Vinícius Jr"),
        ];

        NationalTeam::new(Team::BRA, stk)
    }

    pub fn qatar() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Marquinhos"),
            Sticker::new("2", "Neymar"),
            Sticker::new("3", "Marquinhos"),
            Sticker::new("4", "Neymar"),
        ];

        NationalTeam::new(Team::QAT, stk)
    }

    pub fn ecuador() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Marquinhos"),
            Sticker::new("2", "Neymar"),
            Sticker::new("3", "Marquinhos"),
            Sticker::new("4", "Neymar"),
        ];

        NationalTeam::new(Team::ECU, stk)
    }

    pub fn senegal() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Marquinhos"),
            Sticker::new("2", "Neymar"),
            Sticker::new("3", "Marquinhos"),
            Sticker::new("4", "Neymar"),
        ];

        NationalTeam::new(Team::SEN, stk)
    }

    pub fn netherlands() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Marquinhos"),
            Sticker::new("2", "Neymar"),
            Sticker::new("3", "Marquinhos"),
            Sticker::new("4", "Neymar"),
        ];

        NationalTeam::new(Team::NED, stk)
    }

    pub fn england() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Pickford"),
            Sticker::new("1", "Team"),
        ];
        NationalTeam::new(Team::ENG, stk)
    }

    pub fn usa() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Pickford"),
            Sticker::new("1", "Team"),
        ];
        NationalTeam::new(Team::USA, stk)
    }

    pub fn iran() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Pickford"),
            Sticker::new("1", "Team"),
        ];
        NationalTeam::new(Team::IRN, stk)
    }

    pub fn wales() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Pickford"),
            Sticker::new("1", "Team"),
        ];
        NationalTeam::new(Team::WAL, stk)
    }

    pub fn collect<'a>(&mut self, id: &'a str) {
        let mut sticker = *self.stickers.get_mut(id).unwrap();
        sticker.collect();
        *self.stickers.get_mut(id).unwrap() = sticker;
    }

    pub fn trade<'a>(&mut self, id: &'a str) -> bool {
        let mut sticker = *self.stickers.get_mut(id).unwrap();
        let res = sticker.trade();
        *self.stickers.get_mut(id).unwrap() = sticker;
        return res;
    }
}
