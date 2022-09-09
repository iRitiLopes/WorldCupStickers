#[path = "./album/album.rs"]
mod album;

#[path = "./album/national_team.rs"]
mod national_team;

#[path = "./cli/commands.rs"]
mod commands;

#[path = "./album/team.rs"]
mod team;

mod sticker;
fn main() {
    let command = commands::Cli::parse(&mut std::env::args());
    let mut album = album::Album::new();
    command.execute(&mut album);
}
