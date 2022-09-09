use core::hash::Hash;
use core::str::FromStr;

use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Team {
    FWC,
    QAT,
    ECU,
    SEN,
    NED,
    ENG,
    USA,
    IRN,
    WAL,
    ARG,
    KSA,
    MEX,
    POL,
    FRA,
    DEN,
    TUN,
    AUS,
    SPA,
    CRC,
    GER,
    JPN,
    BEL,
    CAN,
    MOR,
    CRO,
    BRA,
    SRB,
    SWI,
    CMR,
    POR,
    GHA,
    URU,
    KOR,
}

impl Copy for Team {}

impl Clone for Team {
    fn clone(&self) -> Self {
        match self {
            Self::FWC => Self::FWC,
            Team::QAT => Team::QAT,
            Team::ECU => Team::ECU,
            Team::SEN => Team::SEN,
            Team::NED => Team::NED,
            Team::ENG => Team::ENG,
            Team::USA => Team::USA,
            Team::IRN => Team::IRN,
            Team::WAL => Team::WAL,
            Team::ARG => Team::ARG,
            Team::KSA => Team::KSA,
            Team::MEX => Team::MEX,
            Team::POL => Team::POL,
            Team::FRA => Team::FRA,
            Team::DEN => Team::DEN,
            Team::TUN => Team::TUN,
            Team::AUS => Team::AUS,
            Team::SPA => Team::SPA,
            Team::CRC => Team::CRC,
            Team::GER => Team::GER,
            Team::JPN => Team::JPN,
            Team::BEL => Team::BEL,
            Team::CAN => Team::CAN,
            Team::MOR => Team::MOR,
            Team::CRO => Team::CRO,
            Team::BRA => Team::BRA,
            Team::SRB => Team::SRB,
            Team::SWI => Team::SWI,
            Team::CMR => Team::CMR,
            Team::POR => Team::POR,
            Team::GHA => Team::GHA,
            Team::URU => Team::URU,
            Team::KOR => Team::KOR,
        }
    }
}

impl FromStr for Team {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FWC" | "fwc" => Ok(Team::FWC),
            "QAT" | "qat" => Ok(Team::QAT),
            "ECU" | "ecu" => Ok(Team::ECU),
            "SEN" | "sen" => Ok(Team::SEN),
            "NED" | "ned" => Ok(Team::NED),
            "ENG" | "eng" => Ok(Team::ENG),
            "USA" | "usa" => Ok(Team::USA),
            "IRN" | "irn" => Ok(Team::IRN),
            "WAL" | "wal" => Ok(Team::WAL),
            "ARG" | "arg" => Ok(Team::ARG),
            "KSA" | "ksa" => Ok(Team::KSA),
            "MEX" | "mex" => Ok(Team::MEX),
            "POL" | "pol" => Ok(Team::POL),
            "FRA" | "fra" => Ok(Team::FRA),
            "DEN" | "den" => Ok(Team::DEN),
            "TUN" | "tun" => Ok(Team::TUN),
            "AUS" | "aus" => Ok(Team::AUS),
            "SPA" | "spa" => Ok(Team::SPA),
            "CRC" | "crc" => Ok(Team::CRC),
            "GER" | "ger" => Ok(Team::GER),
            "JPN" | "jpn" => Ok(Team::JPN),
            "BEL" | "bel" => Ok(Team::BEL),
            "CAN" | "can" => Ok(Team::CAN),
            "MOR" | "mor" => Ok(Team::MOR),
            "CRO" | "cro" => Ok(Team::CRO),
            "BRA" | "bra" => Ok(Team::BRA),
            "SRB" | "srb" => Ok(Team::SRB),
            "SWI" | "swi" => Ok(Team::SWI),
            "CMR" | "cmr" => Ok(Team::CMR),
            "POR" | "por" => Ok(Team::POR),
            "GHA" | "gha" => Ok(Team::GHA),
            "URU" | "uru" => Ok(Team::URU),
            "KOR" | "kor" => Ok(Team::KOR),
            _ => Err(()),
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
            (Team::ENG, Team::ENG) => true,
            (Team::USA, Team::USA) => true,
            (Team::IRN, Team::IRN) => true,
            (Team::WAL, Team::WAL) => true,
            (Team::ARG, Team::ARG) => true,
            (Team::KSA, Team::KSA) => true,
            (Team::MEX, Team::MEX) => true,
            (Team::POL, Team::POL) => true,
            (Team::FRA, Team::FRA) => true,
            (Team::DEN, Team::DEN) => true,
            (Team::TUN, Team::TUN) => true,
            (Team::AUS, Team::AUS) => true,
            (Team::SPA, Team::SPA) => true,
            (Team::CRC, Team::CRC) => true,
            (Team::GER, Team::GER) => true,
            (Team::JPN, Team::JPN) => true,
            (Team::BEL, Team::BEL) => true,
            (Team::CAN, Team::CAN) => true,
            (Team::MOR, Team::MOR) => true,
            (Team::CRO, Team::CRO) => true,
            (Team::BRA, Team::BRA) => true,
            (Team::SRB, Team::SRB) => true,
            (Team::SWI, Team::SWI) => true,
            (Team::CMR, Team::CMR) => true,
            (Team::POR, Team::POR) => true,
            (Team::GHA, Team::GHA) => true,
            (Team::URU, Team::URU) => true,
            (Team::KOR, Team::KOR) => true,
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
            Team::ENG => state.write_u8(7),
            Team::USA => state.write_u8(8),
            Team::IRN => state.write_u8(9),
            Team::WAL => state.write_u8(10),
            Team::ARG => state.write_u8(11),
            Team::KSA => state.write_u8(12),
            Team::MEX => state.write_u8(13),
            Team::POL => state.write_u8(14),
            Team::FRA => state.write_u8(15),
            Team::DEN => state.write_u8(16),
            Team::TUN => state.write_u8(17),
            Team::AUS => state.write_u8(18),
            Team::SPA => state.write_u8(19),
            Team::CRC => state.write_u8(20),
            Team::GER => state.write_u8(21),
            Team::JPN => state.write_u8(22),
            Team::BEL => state.write_u8(23),
            Team::CAN => state.write_u8(24),
            Team::MOR => state.write_u8(25),
            Team::CRO => state.write_u8(26),
            Team::SRB => state.write_u8(27),
            Team::SWI => state.write_u8(28),
            Team::CMR => state.write_u8(29),
            Team::POR => state.write_u8(30),
            Team::GHA => state.write_u8(31),
            Team::URU => state.write_u8(32),
            Team::KOR => state.write_u8(33),
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Team({:?})", self)
    }
}
