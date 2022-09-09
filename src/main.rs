mod album;
mod cli;
fn main() {
    let command = cli::Cli::parse(&mut std::env::args());
    let mut album = album::Album::new();
    command.execute(&mut album);
}
