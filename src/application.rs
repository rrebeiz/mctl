use crate::MusicInfo;
use clap::{Parser, Subcommand};
use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use serde::Serialize;
#[derive(Debug, PartialEq)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Application {
    player: Option<mpris::Player>,
    pub status: PlayerStatus,
}
#[derive(Debug, Serialize)]
pub struct Response {
    pub text: String,
    pub tooltip: String,
    pub class: String,
}

impl Application {
    pub fn new() -> Self {
        Application {
            player: None,
            status: PlayerStatus::Stopped,
        }
    }
    pub fn refresh_player(&mut self) {
        self.player = PlayerFinder::new().ok().and_then(|f| f.find_active().ok());
    }
    pub fn get_metadata(&mut self) -> Result<Option<MusicInfo>, Box<dyn std::error::Error>> {
        if let Some(player) = &self.player {
            let metadata = player.get_metadata()?;

            let album = match metadata.album_name() {
                Some(a) if !a.trim().is_empty() => a.to_string(),
                _ => return Ok(None),
            };

            let title = match metadata.title() {
                Some(t) if !t.trim().is_empty() => t.to_string(),
                _ => return Ok(None),
            };
            let artists = match metadata.artists() {
                Some(a) if !a.is_empty() => a.join(", "),
                _ => return Ok(None),
            };

            let song_info = format!("{} by {}", title, artists);
            let music_info = MusicInfo {
                album,
                title,
                artist: artists,
                playing: song_info,
            };
            self.status = PlayerStatus::Playing;
            Ok(Some(music_info))
        } else {
            Ok(None)
        }
    }
    pub fn next_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.refresh_player();
        if let Some(player) = &self.player {
            player.next()?;
        }
        // self.player.next()?;
        Ok(())
    }
    pub fn previous_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.refresh_player();
        if let Some(player) = &self.player {
            player.previous()?;
        }
        // self.player.previous()?;
        Ok(())
    }

    pub fn play_pause_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.refresh_player();
        if let Some(player) = &self.player {
            player.play_pause()?;
        }
        // self.player.play_pause()?;
        if self.status == PlayerStatus::Playing {
            self.status = PlayerStatus::Paused
        } else {
            self.status = PlayerStatus::Playing
        }
        Ok(())
    }
    pub fn stop_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.refresh_player();
        if let Some(player) = &self.player {
            player.stop()?;
        }
        // self.player.stop()?;
        self.status = PlayerStatus::Stopped;
        Ok(())
    }

    pub fn json(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.refresh_player();
        let metadata = self.get_metadata()?;
        match metadata {
            Some(info) => {
                let json_str = serde_json::to_string(&info)?;
                Ok(Some(json_str))
            }
            None => Ok(None),
        }
    }
    pub fn waybar(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.refresh_player();
        if let Some(player) = &self.player {
            match player.get_playback_status()? {
                PlaybackStatus::Playing => {
                    self.status = PlayerStatus::Playing;
                    let metadata = self.get_metadata()?.unwrap();

                    let res = Response {
                        text: format!("󰏤 {}", metadata.playing),
                        tooltip: format!("{}", metadata.playing),
                        class: "playing".to_string(),
                    };
                    let json_str = serde_json::to_string(&res)?;
                    Ok(Some(json_str))
                }
                PlaybackStatus::Paused => {
                    self.status = PlayerStatus::Paused;
                    let res = Response {
                        text: "󰐊".to_string(),
                        tooltip: "play".to_string(),
                        class: "paused".to_string(),
                    };
                    let json_str = serde_json::to_string(&res)?;
                    Ok(Some(json_str))
                }
                PlaybackStatus::Stopped => {
                    self.status = PlayerStatus::Stopped;
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Parser)]
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
