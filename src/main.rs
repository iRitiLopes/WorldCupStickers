mod album;
mod cli;
fn main() {
    let stored_album = std::fs::read_to_string(album::Album::path());
    let data: String;
    let mut album: album::Album;
    if stored_album.is_err() {
        album = album::Album::new();
    } else {
        data = stored_album.ok().unwrap();
        album = album::Album::load(&data).ok().unwrap();
    }

    cli::Cli::execute(&mut album);
}
