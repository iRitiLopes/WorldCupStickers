use commands::Command;
use teams::Team;

#[path = "./album/album.rs"]
mod album;

#[path = "./album/teams.rs"]
mod teams;

#[path = "./cli/commands.rs"]
mod commands;

mod sticker;
fn main() {
    let command = commands::Cli::parse(&mut std::env::args());
    let mut a = album::Album::new();
    command.execute(&mut a);
    println!("{}", a)
}
