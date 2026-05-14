pub mod application;
pub mod cli;
use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MusicInfo {
    title: String,
    artist: String,
    album: String,
    pub playing: String,
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Options {
    #[command(subcommand)]
    pub command: Option<PlayerCommands>,
}

#[derive(Subcommand)]
pub enum PlayerCommands {
    /// Play the next song
    #[command(visible_alias = "n")]
    Next,
    /// Play the previous song
    #[command(visible_alias = "b")]
    Previous,
    /// Play / Pause the current song
    #[command(visible_alias = "p")]
    Pause,
    /// Stop the song
    #[command(visible_alias = "s")]
    Stop,
    /// Get JSON info
    #[command(visible_alias = "j")]
    Json,
    /// Get Status
    #[command(visible_alias = "w")]
    Waybar,
}
