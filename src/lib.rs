pub mod application;
pub mod cli;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MusicInfo {
    title: String,
    artist: String,
    album: String,
    pub playing: String,
}
