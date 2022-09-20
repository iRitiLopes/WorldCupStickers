

use std::{collections::HashMap, vec};

use serde::{Deserialize, Serialize};

use super::team::Team;
use super::sticker::Sticker;

#[derive(Serialize, Deserialize)]
pub struct NationalTeam<'a> {
    team: Team,

    #[serde(borrow)]
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

    

    pub fn qatar() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Saad Al Sheeb"),
            Sticker::new("4", "Meshaal Barsham"),
            Sticker::new("5", "Homam Ahmed"),
            Sticker::new("6", "Bassam Alrawi"),
            Sticker::new("7", "Abdulkarim Hassan"),
            Sticker::new("8", "Musaab Khidir"),
            Sticker::new("9", "Boualem Khoukhi"),
            Sticker::new("10", "Pedro Miguel"),
            Sticker::new("11", "Tarek Salman"),
            Sticker::new("12", "Karim Boudiaf"),
            Sticker::new("13", "Abdulaziz Hatem"),
            Sticker::new("14", "Assim Omer Madibo"),
            Sticker::new("15", "Yousuf Abdurisag"),
            Sticker::new("16", "Akram Hassan Afif"),
            Sticker::new("17", "Ahmad Alaaeldin"),
            Sticker::new("18", "Hasan Al-Haydos"),
            Sticker::new("19", "Almoez Ali"),
            Sticker::new("20", "Mohammed Muntari"),
        ];

        NationalTeam::new(Team::QAT, stk)
    }

    pub fn ecuador() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Hernán Galíndez"),
            Sticker::new("4", "Alexander Domínguez"),
            Sticker::new("5", "Robert Arboleda"),
            Sticker::new("6", "Byron Castillo"),
            Sticker::new("7", "Pervis Estupiñán"),
            Sticker::new("8", "Piero Hincapié"),
            Sticker::new("9", "Ángelo Preciado"),
            Sticker::new("10", "Félix Torres"),
            Sticker::new("11", "Moisés Caicedo"),
            Sticker::new("12", "Alan Franco"),
            Sticker::new("13", "Carlos Gruezo"),
            Sticker::new("14", "Jhegson Méndez"),
            Sticker::new("15", "Jeremy Sarmiento"),
            Sticker::new("16", "Michael Estrada"),
            Sticker::new("17", "Ángel Mena"),
            Sticker::new("18", "Gonzalo Plata"),
            Sticker::new("19", "Ayrton Preciado"),
            Sticker::new("20", "Enner Valencia"),
        ];

        NationalTeam::new(Team::ECU, stk)
    }

    pub fn senegal() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Édouard Mendy"),
            Sticker::new("4", "Alfred Gomis"),
            Sticker::new("5", "Saliou Ciss"),
            Sticker::new("6", "Pape Abou Cissé"),
            Sticker::new("7", "Abdou Diallo"),
            Sticker::new("8", "Kalidou Koulibaly"),
            Sticker::new("9", "Ibrahima Mbaye"),
            Sticker::new("10", "Bouna Sarr"),
            Sticker::new("11", "Krépin Diatta"),
            Sticker::new("12", "Idrissa Gueye"),
            Sticker::new("13", "Pape Gueye"),
            Sticker::new("14", "Cheikhou Kouyaté"),
            Sticker::new("15", "Nampalys Mendy"),
            Sticker::new("16", "Boulaye Dia"),
            Sticker::new("17", "Famara Diédhiou"),
            Sticker::new("18", "Bamba Dieng"),
            Sticker::new("19", "Sadio Mané"),
            Sticker::new("20", "Ismaïla Sarr"),
        ];

        NationalTeam::new(Team::SEN, stk)
    }

    pub fn netherlands() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Justin Bijlow"),
            Sticker::new("4", "Jasper Cillessen"),
            Sticker::new("5", "Daley Blind"),
            Sticker::new("6", "Matthijs de Ligt"),
            Sticker::new("7", "Stefan de Vrij"),
            Sticker::new("8", "Denzel Dumfries"),
            Sticker::new("9", "Virgil van Dijk"),
            Sticker::new("10", "Steven Berghuis"),
            Sticker::new("11", "Frenkie de Jong"),
            Sticker::new("12", "Ryan Gravenberch"),
            Sticker::new("13", "Davy Klaassen"),
            Sticker::new("14", "Teun Koopmeiners"),
            Sticker::new("15", "Georginio Wijnaldum"),
            Sticker::new("16", "Steven Bergwijn"),
            Sticker::new("17", "Arnaut Danjuma"),
            Sticker::new("18", "Memphis Depay"),
            Sticker::new("19", "Cody Gakpo"),
            Sticker::new("20", "Donyell Malen"),
        ];

        NationalTeam::new(Team::NED, stk)
    }

    pub fn england() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Jordan Pickford"),
            Sticker::new("4", "Aaron Ramsdale"),
            Sticker::new("5", "Trent Alexander-Arnold"),
            Sticker::new("6", "Conor Coady"),
            Sticker::new("7", "Harry Maguire"),
            Sticker::new("8", "Luke Shaw"),
            Sticker::new("9", "John Stones"),
            Sticker::new("10", "Kyle Walker"),
            Sticker::new("11", "Jude Bellingham"),
            Sticker::new("12", "Jack Grealish"),
            Sticker::new("13", "Jordan Henderson"),
            Sticker::new("14", "Mason Mount"),
            Sticker::new("15", "Kalvin Phillips"),
            Sticker::new("16", "Declan Rice"),
            Sticker::new("17", "Phil Foden"),
            Sticker::new("18", "Harry Kane"),
            Sticker::new("19", "Bukayo Saka"),
            Sticker::new("20", "Raheem Sterling"),
        ];
        NationalTeam::new(Team::ENG, stk)
    }

    pub fn usa() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Matt Turner"),
            Sticker::new("4", "Zack Steffen"),
            Sticker::new("5", "Sergiño Dest"),
            Sticker::new("6", "Aaron Long"),
            Sticker::new("7", "Chris Richards"),
            Sticker::new("8", "Antonee Robinson"),
            Sticker::new("9", "DeAndre Yedlin"),
            Sticker::new("10", "Walker Zimmerman"),
            Sticker::new("11", "Brenden Aaronson"),
            Sticker::new("12", "Kellyn Acosta"),
            Sticker::new("13", "Tyler Adams"),
            Sticker::new("14", "Weston McKennie"),
            Sticker::new("15", "Yunus Musah"),
            Sticker::new("16", "Jesús Ferreira"),
            Sticker::new("17", "Ricardo Pepi"),
            Sticker::new("18", "Christian Pulisic"),
            Sticker::new("19", "Giovanni Reyna"),
            Sticker::new("20", "Timothy Weah"),
        ];
        NationalTeam::new(Team::USA, stk)
    }

    pub fn iran() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Amir Abedzadeh"),
            Sticker::new("4", "Alireza Beiranvand"),
            Sticker::new("5", "Ehsan Hajsafi"),
            Sticker::new("6", "Majid Hosseini"),
            Sticker::new("7", "Hossein Kanaani"),
            Sticker::new("8", "Shoja Khalilzadeh"),
            Sticker::new("9", "Milad Mohammadi"),
            Sticker::new("10", "Sadegh Moharrami"),
            Sticker::new("11", "Omid Noorafkan"),
            Sticker::new("12", "Vahid Amiri"),
            Sticker::new("13", "Saeid Ezatolahi"),
            Sticker::new("14", "Ali Gholizadeh"),
            Sticker::new("15", "Alireza Jahanbakhsh"),
            Sticker::new("16", "Ahmad Nourollahi"),
            Sticker::new("17", "Karim Ansarifard"),
            Sticker::new("18", "Sardar Azmoun"),
            Sticker::new("19", "Saman Ghoddos"),
            Sticker::new("20", "Mehdi Taremi"),
        ];
        NationalTeam::new(Team::IRN, stk)
    }

    pub fn wales() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Danny Ward"),
            Sticker::new("4", "Wayne Hennessey"),
            Sticker::new("5", "Ethan Ampadu"),
            Sticker::new("6", "Ben Davies"),
            Sticker::new("7", "Chris Gunter"),
            Sticker::new("8", "Chris Mepham"),
            Sticker::new("9", "Connor Roberts"),
            Sticker::new("10", "Joe Rodon"),
            Sticker::new("11", "Neco Williams"),
            Sticker::new("12", "Joe Allen"),
            Sticker::new("13", "Joe Morrell"),
            Sticker::new("14", "Aaron Ramsey"),
            Sticker::new("15", "Jonathan Williams"),
            Sticker::new("16", "Harry Wilson"),
            Sticker::new("17", "Gareth Bale"),
            Sticker::new("18", "Daniel James"),
            Sticker::new("19", "Brennan Johnson"),
            Sticker::new("20", "Kieffer Moore"),
        ];
        NationalTeam::new(Team::WAL, stk)
    }

    pub fn argentina() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Emiliano Martínez"),
            Sticker::new("4", "Franco Armani"),
            Sticker::new("5", "Marcos Acuña"),
            Sticker::new("6", "Nahuel Molina"),
            Sticker::new("7", "Nicolás Otamendi"),
            Sticker::new("8", "Germán Pezzella"),
            Sticker::new("9", "Cristian Romero"),
            Sticker::new("10", "Rodrigo De Paul"),
            Sticker::new("11", "Ángel Di María"),
            Sticker::new("12", "Giovani Lo Celso"),
            Sticker::new("13", "Leandro Paredes"),
            Sticker::new("14", "Guido Rodríguez"),
            Sticker::new("15", "Julián Álvarez"),
            Sticker::new("16", "Joaquín Correa"),
            Sticker::new("17", "Alejandro Gómez"),
            Sticker::new("18", "Nicolás González"),
            Sticker::new("19", "Lautaro Martínez"),
            Sticker::new("20", "Lionel Messi"),
        ];
        NationalTeam::new(Team::ARG, stk)
    }

    pub fn saudi_arabia() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Mohammed Al-Owais"),
            Sticker::new("4", "Mohammed Al-Rubaie"),
            Sticker::new("5", "Abdulelah Al-Amri"),
            Sticker::new("6", "Ali Al-Boleahi"),
            Sticker::new("7", "Mohammed Al-Burayk"),
            Sticker::new("8", "Sultan Al-Ghannam"),
            Sticker::new("9", "Yasser Al-Shahrani"),
            Sticker::new("10", "Hassan Al-Tambakti"),
            Sticker::new("11", "Abdullah Madu"),
            Sticker::new("12", "Salman Al-Faraj"),
            Sticker::new("13", "Abdulelah Al-Malki"),
            Sticker::new("14", "Sami Al-Najei"),
            Sticker::new("15", "Hattan Bahebri"),
            Sticker::new("16", "Mohamed Kanno"),
            Sticker::new("17", "Abdullah Otayf"),
            Sticker::new("18", "Firas Al-Buraikan"),
            Sticker::new("19", "Salem Al-Dawsari"),
            Sticker::new("20", "Khalid Al-Ghannam"),
        ];
        NationalTeam::new(Team::KSA, stk)
    }

    pub fn mexico() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Guillermo Ochoa"),
            Sticker::new("4", "Alfredo Talavera"),
            Sticker::new("5", "Néstor Araújo"),
            Sticker::new("6", "Jesús Gallardo"),
            Sticker::new("7", "César Montes"),
            Sticker::new("8", "Héctor Moreno"),
            Sticker::new("9", "Luis Romo"),
            Sticker::new("10", "Jorge Sánchez"),
            Sticker::new("11", "Edson Álvarez"),
            Sticker::new("12", "Jesús Manuel Corona"),
            Sticker::new("13", "Andrés Guardado"),
            Sticker::new("14", "Érick Gutiérrez"),
            Sticker::new("15", "Héctor Herrera"),
            Sticker::new("16", "Diego Lainez"),
            Sticker::new("17", "Carlos Rodríguez"),
            Sticker::new("18", "Rogelio Funes Mori"),
            Sticker::new("19", "Raúl Jiménez"),
            Sticker::new("20", "Hirving Lozano"),
        ];
        NationalTeam::new(Team::MEX, stk)
    }

    pub fn poland() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Wojciech Szczęsny"),
            Sticker::new("4", "Łukasz Skorupski"),
            Sticker::new("5", "Jan Bednarek"),
            Sticker::new("6", "Bartosz Bereszyński"),
            Sticker::new("7", "Matty Cash"),
            Sticker::new("8", "Kamil Glik"),
            Sticker::new("9", "Tymoteusz Puchacz"),
            Sticker::new("10", "Maciej Rybus"),
            Sticker::new("11", "Kamil Jóźwiak"),
            Sticker::new("12", "Mateusz Klich"),
            Sticker::new("13", "Grzegorz Krychowiak"),
            Sticker::new("14", "Jakub Moder"),
            Sticker::new("15", "Sebastian Szymański"),
            Sticker::new("16", "Piotr Zieliński"),
            Sticker::new("17", "Robert Lewandowski"),
            Sticker::new("18", "Arkadiusz Milik"),
            Sticker::new("19", "Krzysztof Piątek"),
            Sticker::new("20", "Karol Świderski"),
        ];
        NationalTeam::new(Team::POL, stk)
    }

    pub fn france() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Hugo Lloris"),
            Sticker::new("4", "Mike Maignan"),
            Sticker::new("5", "Lucas Hernández"),
            Sticker::new("6", "Theo Hernández"),
            Sticker::new("7", "Presnel Kimpembe"),
            Sticker::new("8", "Jules Koundé"),
            Sticker::new("9", "Benjamin Pavard"),
            Sticker::new("10", "Raphaël Varane"),
            Sticker::new("11", "N’Golo Kanté"),
            Sticker::new("12", "Paul Pogba"),
            Sticker::new("13", "Adrien Rabiot"),
            Sticker::new("14", "Aurélien Tchouaméni"),
            Sticker::new("15", "Wissam Ben Yedder"),
            Sticker::new("16", "Karim Benzema"),
            Sticker::new("17", "Kingsley Coman"),
            Sticker::new("18", "Antoine Griezmann"),
            Sticker::new("19", "Kylian Mbappé"),
            Sticker::new("20", "Christopher Nkunku"),
        ];
        NationalTeam::new(Team::FRA, stk)
    }

    pub fn australia() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Mathew Ryan"),
            Sticker::new("4", "Andrew Redmayne"),
            Sticker::new("5", "Aziz Behich"),
            Sticker::new("6", "Miloš Degenek"),
            Sticker::new("7", "Rhyan Grant"),
            Sticker::new("8", "Joel King"),
            Sticker::new("9", "Trent Sainsbury"),
            Sticker::new("10", "Harry Souttar"),
            Sticker::new("11", "Ajdin Hrustić"),
            Sticker::new("12", "Jackson Irvine"),
            Sticker::new("13", "James Jeggo"),
            Sticker::new("14", "Awer Mabil"),
            Sticker::new("15", "Aaron Mooy"),
            Sticker::new("16", "Martin Boyle"),
            Sticker::new("17", "Mitchell Duke"),
            Sticker::new("18", "Craig Goodwin"),
            Sticker::new("19", "Mathew Leckie"),
            Sticker::new("20", "Adam Taggart"),
        ];
        NationalTeam::new(Team::AUS, stk)
    }

    pub fn denmark() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Kasper Schmeichel"),
            Sticker::new("4", "Frederik Rønnow"),
            Sticker::new("5", "Andreas Christensen"),
            Sticker::new("6", "Simon Kjær"),
            Sticker::new("7", "Joakim Mæhle"),
            Sticker::new("8", "Jens Stryger Larsen"),
            Sticker::new("9", "Jannik Vestergaard"),
            Sticker::new("10", "Mikkel Damsgaard"),
            Sticker::new("11", "Thomas Delaney"),
            Sticker::new("12", "Christian Eriksen"),
            Sticker::new("13", "Pierre Emile Højbjerg"),
            Sticker::new("14", "Christian Nørgaard"),
            Sticker::new("15", "Daniel Wass"),
            Sticker::new("16", "Martin Braithwaite"),
            Sticker::new("17", "Kasper Dolberg"),
            Sticker::new("18", "Yussuf Poulsen"),
            Sticker::new("19", "Andreas Skov Olsen"),
            Sticker::new("20", "Jonas Wind"),
        ];
        NationalTeam::new(Team::DEN, stk)
    }

    pub fn tunisia() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Bechir Ben Saïd"),
            Sticker::new("4", "Farouk Ben Mustapha"),
            Sticker::new("5", "Dylan Bronn"),
            Sticker::new("6", "Mohamed Dräger"),
            Sticker::new("7", "Bilel Ifa"),
            Sticker::new("8", "Ali Maâloul"),
            Sticker::new("9", "Hamza Mathlouthi"),
            Sticker::new("10", "Yassine Meriah"),
            Sticker::new("11", "Montassar Talbi"),
            Sticker::new("12", "Mohamed Ali Ben Romdhane"),
            Sticker::new("13", "Wahbi Khazri"),
            Sticker::new("14", "Aïssa Laïdouni"),
            Sticker::new("15", "Ferjani Sassi"),
            Sticker::new("16", "Ellyes Skhiri"),
            Sticker::new("17", "Anis Slimane"),
            Sticker::new("18", "Seifeddine Jaziri"),
            Sticker::new("19", "Youssef Msakni"),
            Sticker::new("20", "Naïm Sliti"),
        ];
        NationalTeam::new(Team::TUN, stk)
    }

    pub fn spain() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Unai Simón"),
            Sticker::new("4", "Robert Sánchez"),
            Sticker::new("5", "César Azpilicueta"),
            Sticker::new("6", "Eric García"),
            Sticker::new("7", "Jordi Alba"),
            Sticker::new("8", "Aymeric Laporte"),
            Sticker::new("9", "Pau Torres"),
            Sticker::new("10", "Gavi"),
            Sticker::new("11", "Koke"),
            Sticker::new("12", "Marcos Llorente"),
            Sticker::new("13", "Pedri"),
            Sticker::new("14", "Rodri"),
            Sticker::new("15", "Sergio Busquets"),
            Sticker::new("16", "Dani Olmo"),
            Sticker::new("17", "Ansu Fati"),
            Sticker::new("18", "Ferran Torres"),
            Sticker::new("19", "Álvaro Morata"),
            Sticker::new("20", "Pablo Sarabia"),
        ];
        NationalTeam::new(Team::SPA, stk)
    }

    pub fn costa_rica() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Keylor Navas"),
            Sticker::new("4", "Leonel Moreira"),
            Sticker::new("5", "Ricardo Blanco"),
            Sticker::new("6", "Francisco Calvo"),
            Sticker::new("7", "Óscar Duarte"),
            Sticker::new("8", "Keysher Fuller"),
            Sticker::new("9", "Bryan Oviedo"),
            Sticker::new("10", "Juan Pablo Vargas"),
            Sticker::new("11", "Kendall Waston"),
            Sticker::new("12", "Celso Borges"),
            Sticker::new("13", "Orlando Galo"),
            Sticker::new("14", "Bryan Ruiz"),
            Sticker::new("15", "Yeltsin Tejeda"),
            Sticker::new("16", "Jewison Bennette"),
            Sticker::new("17", "Joel Campbell"),
            Sticker::new("18", "Anthony Contreras"),
            Sticker::new("19", "Gerson Torres"),
            Sticker::new("20", "Johan Venegas"),
        ];
        NationalTeam::new(Team::CRC, stk)
    }

    pub fn germany() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Manuel Neuer"),
            Sticker::new("4", "Marc-André ter Stegen"),
            Sticker::new("5", "Matthias Ginter"),
            Sticker::new("6", "Robin Gosens"),
            Sticker::new("7", "Thilo Kehrer"),
            Sticker::new("8", "David Raum"),
            Sticker::new("9", "Antonio Rüdiger"),
            Sticker::new("10", "Niklas Süle"),
            Sticker::new("11", "Leon Goretzka"),
            Sticker::new("12", "İlkay Gündoğan"),
            Sticker::new("13", "Kai Havertz"),
            Sticker::new("14", "Jonas Hofmann"),
            Sticker::new("15", "Joshua Kimmich"),
            Sticker::new("16", "Serge Gnabry"),
            Sticker::new("17", "Thomas Müller"),
            Sticker::new("18", "Marco Reus"),
            Sticker::new("19", "Leroy Sané"),
            Sticker::new("20", "Timo Werner"),
        ];
        NationalTeam::new(Team::GER, stk)
    }

    pub fn japan() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Shuichi Gonda"),
            Sticker::new("4", "Eiji Kawashima"),
            Sticker::new("5", "Yuto Nagatomo"),
            Sticker::new("6", "Yuta Nakayama"),
            Sticker::new("7", "Takehiro Tomiyasu"),
            Sticker::new("8", "Miki Yamane"),
            Sticker::new("9", "Maya Yoshida"),
            Sticker::new("10", "Wataru Endo"),
            Sticker::new("11", "Genki Haraguchi"),
            Sticker::new("12", "Hidemasa Morita"),
            Sticker::new("13", "Gaku Shibasaki"),
            Sticker::new("14", "Ao Tanaka"),
            Sticker::new("15", "Takuma Asano"),
            Sticker::new("16", "Kyogo Furuhashi"),
            Sticker::new("17", "Junya Ito"),
            Sticker::new("18", "Takumi Minamino"),
            Sticker::new("19", "Kaoru Mitoma"),
            Sticker::new("20", "Yuya Osako"),
        ];
        NationalTeam::new(Team::JPN, stk)
    }

    pub fn belgica() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Thibaut Courtois"),
            Sticker::new("4", "Simon Mignolet"),
            Sticker::new("5", "Toby Alderweireld"),
            Sticker::new("6", "Timothy Castagne"),
            Sticker::new("7", "Jason Denayer"),
            Sticker::new("8", "Thomas Meunier"),
            Sticker::new("9", "Jan Vertonghen"),
            Sticker::new("10", "Yannick Carrasco"),
            Sticker::new("11", "Kevin De Bruyne"),
            Sticker::new("12", "Charles De Ketelaere"),
            Sticker::new("13", "Thorgan Hazard"),
            Sticker::new("14", "Youri Tielemans"),
            Sticker::new("15", "Hans Vanaken"),
            Sticker::new("16", "Axel Witsel"),
            Sticker::new("17", "Jérémy Doku"),
            Sticker::new("18", "Eden Hazard"),
            Sticker::new("19", "Romelu Lukaku"),
            Sticker::new("20", "Dries Mertens"),
        ];
        NationalTeam::new(Team::BEL, stk)
    }

    pub fn canada() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Milan Borjan"),
            Sticker::new("4", "Maxime Crépeau"),
            Sticker::new("5", "Samuel Adekugbe"),
            Sticker::new("6", "Doneil Henry"),
            Sticker::new("7", "Alistair Johnston"),
            Sticker::new("8", "Richie Laryea"),
            Sticker::new("9", "Kamal Miller"),
            Sticker::new("10", "Steven Vitória"),
            Sticker::new("11", "Tajon Buchanan"),
            Sticker::new("12", "Alphonso Davies"),
            Sticker::new("13", "Stephen Eustáquio"),
            Sticker::new("14", "Atiba Hutchinson"),
            Sticker::new("15", "Mark-Anthony Kaye"),
            Sticker::new("16", "Jonathan Osorio"),
            Sticker::new("17", "Samuel Piette"),
            Sticker::new("18", "Jonathan David"),
            Sticker::new("19", "David Junior Hoilett"),
            Sticker::new("20", "Cyle Larin"),
        ];
        NationalTeam::new(Team::CAN, stk)
    }

    pub fn morocco() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Yassine Bounou"),
            Sticker::new("4", "Munir Mohamedi"),
            Sticker::new("5", "Nayef Aguerd"),
            Sticker::new("6", "Achraf Hakimi"),
            Sticker::new("7", "Adam Masina"),
            Sticker::new("8", "Samy Mmaee"),
            Sticker::new("9", "Romain Saïss"),
            Sticker::new("10", "Selim Amallah"),
            Sticker::new("11", "Sofyan Amrabat"),
            Sticker::new("12", "Aymen Barkok"),
            Sticker::new("13", "Ilias Chair"),
            Sticker::new("14", "Imrân Louza"),
            Sticker::new("15", "Sofiane Boufal"),
            Sticker::new("16", "Ayoub El Kaabi"),
            Sticker::new("17", "Youssef En-Nesyri"),
            Sticker::new("18", "Ryan Mmaee"),
            Sticker::new("19", "Munir"),
            Sticker::new("20", "Tarik Tissoudali"),
        ];
        NationalTeam::new(Team::MOR, stk)
    }

    pub fn croatia() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Dominik Livaković"),
            Sticker::new("4", "Ivica Ivušić"),
            Sticker::new("5", "Duje Ćaleta-Car"),
            Sticker::new("6", "Joško Gvardiol"),
            Sticker::new("7", "Josip Juranović"),
            Sticker::new("8", "Dejan Lovren"),
            Sticker::new("9", "Borna Sosa"),
            Sticker::new("10", "Domagoj Vida"),
            Sticker::new("11", "Marcelo Brozović"),
            Sticker::new("12", "Mateo Kovačić"),
            Sticker::new("13", "Luka Modrić"),
            Sticker::new("14", "Mario Pašalić"),
            Sticker::new("15", "Ivan Perišić"),
            Sticker::new("16", "Nikola Vlašić"),
            Sticker::new("17", "Josip Brekalo"),
            Sticker::new("18", "Andrej Kramarić"),
            Sticker::new("19", "Marko Livaja"),
            Sticker::new("20", "Mislav Oršić"),
        ];
        NationalTeam::new(Team::CRO, stk)
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
            Sticker::new("15", "Antony"),
            Sticker::new("16", "Gabriel Jesus"),
            Sticker::new("17", "Neymar Jr"),
            Sticker::new("18", "Raphinha"),
            Sticker::new("19", "Richarlison"),
            Sticker::new("20", "Vinícius Jr"),
        ];

        NationalTeam::new(Team::BRA, stk)
    }

    pub fn serbia() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Predrag Rajković"),
            Sticker::new("4", "Vanja Milinković-Savić"),
            Sticker::new("5", "Nikola Milenković"),
            Sticker::new("6", "Strahinja Pavlović"),
            Sticker::new("7", "Miloš Veljković"),
            Sticker::new("8", "Filip Đuričić"),
            Sticker::new("9", "Nemanja Gudelj"),
            Sticker::new("10", "Filip Kostić"),
            Sticker::new("11", "Darko Lazović"),
            Sticker::new("12", "Saša Lukić"),
            Sticker::new("13", "Nemanja Maksimović"),
            Sticker::new("14", "Sergej Milinković-Savić"),
            Sticker::new("15", "Nemanja Radonjić"),
            Sticker::new("16", "Andrija Živković"),
            Sticker::new("17", "Luka Jović"),
            Sticker::new("18", "Aleksandar Mitrović"),
            Sticker::new("19", "Dušan Tadić"),
            Sticker::new("20", "Dušan Vlahović"),
        ];
        NationalTeam::new(Team::SRB, stk)
    }

    pub fn switzerland() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Yann Sommer"),
            Sticker::new("4", "Gregor Kobel"),
            Sticker::new("5", "Manuel Akanji"),
            Sticker::new("6", "Nico Elvedi"),
            Sticker::new("7", "Kevin Mbabu"),
            Sticker::new("8", "Ricardo Rodríguez"),
            Sticker::new("9", "Fabian Schär"),
            Sticker::new("10", "Silvan Widmer"),
            Sticker::new("11", "Remo Freuler"),
            Sticker::new("12", "Xherdan Shaqiri"),
            Sticker::new("13", "Djibril Sow"),
            Sticker::new("14", "Granit Xhaka"),
            Sticker::new("15", "Denis Zakaria"),
            Sticker::new("16", "Steven Zuber"),
            Sticker::new("17", "Breel Embolo"),
            Sticker::new("18", "Noah Okafor"),
            Sticker::new("19", "Haris Seferović"),
            Sticker::new("20", "Ruben Vargas"),
        ];
        NationalTeam::new(Team::SWI, stk)
    }

    pub fn cameroon() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "André Onana "),
            Sticker::new("4", "Devis Epassy "),
            Sticker::new("5", "Jean-Charles Castelletto "),
            Sticker::new("6", "Collins Fai "),
            Sticker::new("7", "Olivier Mbaizo "),
            Sticker::new("8", "Harold Moukoudi "),
            Sticker::new("9", "Michael Ngadeu "),
            Sticker::new("10", "Nouhou "),
            Sticker::new("11", "Martin Hongla "),
            Sticker::new("12", "Pierre Kunde "),
            Sticker::new("13", "James Léa Siliki "),
            Sticker::new("14", "Samuel Oum Gouet "),
            Sticker::new("15", "André-Frank Zambo Anguissa "),
            Sticker::new("16", "Vincent Aboubakar "),
            Sticker::new("17", "Stéphane Bahoken "),
            Sticker::new("18", "Eric Maxim Choupo-Moting "),
            Sticker::new("19", "Nicolas Moumi Ngamaleu "),
            Sticker::new("20", "Karl Toko Ekambi "),
        ];
        NationalTeam::new(Team::CMR, stk)
    }

    pub fn portugal() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Diogo Costa"),
            Sticker::new("4", "Rui Patrício"),
            Sticker::new("5", "João Cancelo"),
            Sticker::new("6", "José Fonte"),
            Sticker::new("7", "Nuno Mendes"),
            Sticker::new("8", "Pepe"),
            Sticker::new("9", "Raphaël Guerreiro"),
            Sticker::new("10", "Rúben Dias"),
            Sticker::new("11", "Bernardo Silva"),
            Sticker::new("12", "Bruno Fernandes"),
            Sticker::new("13", "Danilo Pereira"),
            Sticker::new("14", "João Moutinho"),
            Sticker::new("15", "Renato Sanches"),
            Sticker::new("16", "Rúben Neves"),
            Sticker::new("17", "André Silva"),
            Sticker::new("18", "Cristiano Ronaldo"),
            Sticker::new("19", "Diogo Jota"),
            Sticker::new("20", "Gonçalo Guedes"),
        ];
        NationalTeam::new(Team::POR, stk)
    }

    pub fn ghana() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Joe Wollacott"),
            Sticker::new("4", "Richard Ofori"),
            Sticker::new("5", "Daniel Amartey"),
            Sticker::new("6", "Abdul-Rahman Baba"),
            Sticker::new("7", "Alexander Djiku"),
            Sticker::new("8", "Gideon Mensah"),
            Sticker::new("9", "Jonathan Mensah"),
            Sticker::new("10", "Andy Yiadom"),
            Sticker::new("11", "Iddrisu Baba"),
            Sticker::new("12", "Mohammed Kudus"),
            Sticker::new("13", "Daniel-Kofi Kyereh"),
            Sticker::new("14", "Thomas Partey"),
            Sticker::new("15", "Mubarak Wakaso"),
            Sticker::new("16", "Felix Afena-Gyan"),
            Sticker::new("17", "André Ayew"),
            Sticker::new("18", "Jordan Ayew"),
            Sticker::new("19", "Issahaku Abdul Fatawu"),
            Sticker::new("20", "Kamaldeen Sulemana"),
        ];
        NationalTeam::new(Team::GHA, stk)
    }

    pub fn uruguay() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Fernando Muslera"),
            Sticker::new("4", "Sergio Rochet"),
            Sticker::new("5", "Ronald Araújo"),
            Sticker::new("6", "Martín Cáceres"),
            Sticker::new("7", "José María Giménez"),
            Sticker::new("8", "Diego Godín"),
            Sticker::new("9", "Mathías Olivera"),
            Sticker::new("10", "Matías Viña"),
            Sticker::new("11", "Rodrigo Bentancur"),
            Sticker::new("12", "Giorgian De Arrascaeta"),
            Sticker::new("13", "Nicolás De La Cruz"),
            Sticker::new("14", "Lucas Torreira"),
            Sticker::new("15", "Federico Valverde"),
            Sticker::new("16", "Matías Vecino"),
            Sticker::new("17", "Edinson Cavani"),
            Sticker::new("18", "Darwin Núñez"),
            Sticker::new("19", "Facundo Pellistri"),
            Sticker::new("20", "Luis Suárez"),
        ];
        NationalTeam::new(Team::URU, stk)
    }

    pub fn korea() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("1", "Team"),
            Sticker::new("2", "Shield"),
            Sticker::new("3", "Seung-gyu Kim"),
            Sticker::new("4", "Hyeon-woo Jo"),
            Sticker::new("5", "Chul Hong"),
            Sticker::new("6", "Tae-hwan Kim"),
            Sticker::new("7", "Min-jae Kim"),
            Sticker::new("8", "Young-gwon Kim"),
            Sticker::new("9", "Jin-su Kim"),
            Sticker::new("10", "Yong Lee"),
            Sticker::new("11", "In-beom Hwang"),
            Sticker::new("12", "Woo-young Jung"),
            Sticker::new("13", "Jae-sung Lee"),
            Sticker::new("14", "Seung-ho Paik"),
            Sticker::new("15", "Gue-sung Cho"),
            Sticker::new("16", "Hee-chan Hwang"),
            Sticker::new("17", "Ui-jo Hwang"),
            Sticker::new("18", "Chang-hoon Kwon"),
            Sticker::new("19", "Heung-min Son"),
            Sticker::new("20", "Min-kyu Song"),
        ];
        NationalTeam::new(Team::KOR, stk)
    }

    pub fn fwc() -> NationalTeam<'static> {
        let stk = vec![
            Sticker::new("0", "Panini"),
            Sticker::new("1", "FIFA"),
            Sticker::new("2", "Official Trophy FOIL"),
            Sticker::new("3", "Official Trophy FOIL"),
            Sticker::new("4", "Official Mascot FOIL"),
            Sticker::new("5", "Official Mascot FOIL"),
            Sticker::new("6", "Official Emblem FOIL"),
            Sticker::new("7", "Official Emblem FOIL"),
            Sticker::new("8", "Ahmad Bin Ali Stadium"),
            Sticker::new("9", "Al Janoub Stadium"),
            Sticker::new("10", "Al Thumama Stadium"),
            Sticker::new("11", "Education City Stadium"),
            Sticker::new("12", "Khalifa International Stadium"),
            Sticker::new("13", "Stadium 974"),
            Sticker::new("14", "Al Bayt Stadium outdoor"),
            Sticker::new("15", "Al Bayt Stadium indoor"),
            Sticker::new("16", "Lusail Stadium outdoor"),
            Sticker::new("17", "Lusail Stadium indoor"),
            Sticker::new("18", "Official Match Ball"),
            Sticker::new("19", "Uruguay 1930"),
            Sticker::new("20", "Italy 1938"),
            Sticker::new("21", "Brazil 1958"),
            Sticker::new("22", "England 1966"),
            Sticker::new("23", "Brazil 1970"),
            Sticker::new("24", "Argentina 1978"),
            Sticker::new("25", "Italy 1982"),
            Sticker::new("26", "Germany 1990"),
            Sticker::new("27", "France 1998"),
            Sticker::new("28", "Spain 2010"),
            Sticker::new("29", "France 2018"),
        ];
        NationalTeam::new(Team::FWC, stk)
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

    pub fn show(&self, missing: bool, repeated: bool) {
        let mut message = format!("{}\n", self.team).to_owned();
        for (_sticker_id, sticker) in &self.stickers {
            if (missing && sticker.is_missing()) || (repeated && sticker.have_repeated()) || (!missing && !repeated){
                message.push_str(&format!("\t- {}\n", sticker));
            }
                
        }
        println!("{}", message);
    }

    pub fn clean(&mut self, repeated: bool) {
        for k in self.stickers.iter_mut(){
            if k.1.have_repeated() && repeated {
                k.1.clean();
            }
        }
    }
}
