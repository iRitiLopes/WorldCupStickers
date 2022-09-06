use crate::sticker::Sticker;
use std::{collections::HashMap, hash::Hash, str::FromStr};

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter)]
pub enum Team {
    FWC,
    QAT,
    ECU,
    SEN,
    NED,
    BRA,
}

impl Copy for Team {}

impl Clone for Team {
    fn clone(&self) -> Self {
        match self {
            Self::FWC => Self::FWC,
            Self::QAT => Self::QAT,
            Self::ECU => Self::ECU,
            Self::SEN => Self::SEN,
            Self::NED => Self::NED,
            Self::BRA => Self::BRA,
        }
    }
}

impl FromStr for Team {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FWC" => Ok(Self::FWC),
            "QAT" => Ok(Self::QAT),
            "ECU" => Ok(Self::ECU),
            "SEN" => Ok(Self::SEN),
            "NED" => Ok(Self::NED),
            "BRA" => Ok(Self::BRA),
            _ => Err(())
        }
    }
}

impl Eq for Team {}

impl PartialEq for Team {
    fn eq(&self, another: &Self) -> bool {
        match (self, another) {
            (Team::FWC, Team::FWC) => true,
            (Team::QAT, Team::QAT) => true,
            (Team::ECU, Team::ECU) => true,
            (Team::SEN, Team::SEN) => true,
            (Team::NED, Team::NED) => true,
            (Team::BRA, Team::BRA) => true,
            _ => false,
        }
    }
}

impl Hash for Team {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            Team::FWC => state.write_u8(1),
            Team::QAT => state.write_u8(2),
            Team::ECU => state.write_u8(3),
            Team::SEN => state.write_u8(4),
            Team::NED => state.write_u8(5),
            Team::BRA => state.write_u8(6),
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Team({:?})", self)
    }
}

pub struct NationalTeam<'a> {
    team: Team,
    stickers: HashMap<&'a str, Sticker<'a>>,
}

impl std::fmt::Display for NationalTeam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}\n", self.team);
        for (s_id, s) in &self.stickers {
            write!(f, "\t- {}\n", s);
        }
        write!(f, "")
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
