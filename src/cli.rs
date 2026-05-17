use crate::application::{Application, PlayerStatus};
use clap::{Parser, Subcommand};

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
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut application = Application::new();
    let options = Options::parse();
    match options.command {
        Some(PlayerCommands::Next) => {
            application.next_song()?;
            Ok(())
        }
        Some(PlayerCommands::Previous) => {
            application.previous_song()?;
            Ok(())
        }
        Some(PlayerCommands::Pause) => {
            application.play_pause_song()?;
            Ok(())
        }
        Some(PlayerCommands::Stop) => {
            application.stop_song()?;
            Ok(())
        }
        Some(PlayerCommands::Json) => {
            if let Some(json) = application.json()? {
                println!("{json}");
            }
            Ok(())
        }
        Some(PlayerCommands::Waybar) => {
            let json = application.waybar();
            if application.status == PlayerStatus::Stopped {
                return Ok(());
            } else {
                match json {
                    Ok(Some(j)) => {
                        println!("{j}");
                    }
                    Ok(None) => {
                        println!("");
                    }
                    Err(_) => {
                        println!("");
                    }
                }
                Ok(())
            }
        }
        None => Ok(()),
    }
}
