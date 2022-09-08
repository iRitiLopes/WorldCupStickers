#[path = "./album/album.rs"]
mod album;

#[path = "./album/teams.rs"]
mod teams;

#[path = "./cli/commands.rs"]
mod commands;

mod sticker;
fn main() {
    let command = commands::Cli::parse(&mut std::env::args());
    let mut album = album::Album::new();
    command.execute(&mut album);
}
