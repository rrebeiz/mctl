use crate::{
    Options, PlayerCommands,
    application::{Application, PlayerStatus},
};
use clap::Parser;
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
