use crate::player::{get_player_from_str, PlayerType};
use dirs;
use serde::{Deserialize, Serialize};
use std::{self, env, fs, path::PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    VolumeTypeMismatch,
    SpeedTypeMismatch,
    PlayerTypeMismatch,
    FilterTypeMismatch,
    HelpAsked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    volume_lvl: Option<f32>,
    speed: Option<f32>,
    pub player: PlayerType,
    pub filter: Option<String>,
}

impl Configs {
    pub fn get_volume(&self) -> f32 {
        self.volume_lvl.unwrap_or(100.0)
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.unwrap_or(1.0)
    }


    pub fn get_config_from_args(mut args: env::Args) -> Result<Self, ConfigError> {
        args.next();

        let mut volume: Option<f32> = None;
        let mut speed: Option<f32> = None;
        let mut player: Option<PlayerType> = None;
        let mut filter: Option<String> = None;
        let mut set_as_default = false;

        let arg_parsed: Option<ConfigError> = loop {
            let a = args.next();
            match a {
                Some(arg) => {
                    if arg == "-s" {
                        let speed_arg = args.next();
                        let speed_arg = match speed_arg {
                            Some(v) => v,
                            None => break Some(ConfigError::SpeedTypeMismatch),
                        };
                        let b = speed_arg.parse::<f32>();
                        println!("{:?}", b);
                        match b {
                            Ok(val) => speed = Some(val),
                            Err(_) => break Some(ConfigError::SpeedTypeMismatch),
                        }
                    } else if arg == "-v" {
                        let vol_arg = args.next();
                        let vol_arg = match vol_arg {
                            Some(v) => v,
                            None => break Some(ConfigError::VolumeTypeMismatch),
                        };
                        let b = vol_arg.parse::<f32>();
                        println!("{:?}", b);
                        match b {
                            Ok(val) => volume = Some(val),
                            Err(_) => break Some(ConfigError::VolumeTypeMismatch),
                        }
                    } else if arg == "--h" || arg == "-help" {
                        return Err(ConfigError::HelpAsked);
                    } else if arg == "-player" {
                        let player_arg = args.next();
                        let player_arg = match player_arg {
                            Some(v) => v,
                            None => break Some(ConfigError::PlayerTypeMismatch),
                        };
                        player = Some(get_player_from_str(&player_arg));
                    } else if arg == "-f" {
                        let filter_arg = args.next();
                        filter = match filter_arg {
                            Some(v) => Some(v),
                            None => break Some(ConfigError::FilterTypeMismatch),
                        };
                    } else if arg == "--set-default" {
                        set_as_default = true;
                    }
                }
                None => break None,
            }
        };

        match arg_parsed {
            Some(er) => return Err(er),
            None => {}
        }

        let config = match Self::get_default_config() {
            Ok(default) => Configs {
                volume_lvl: Some(volume.unwrap_or_else(|| default.get_volume())),
                speed: Some(speed.unwrap_or_else(|| default.get_speed())),
                player: player.unwrap_or_else(|| default.player),
                filter 
            },
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => (),
                    _ => {
                        // TODO: Do something on not able to open read default config
                    }
                };
                Configs {
                    volume_lvl: volume,
                    speed,
                    player: player.unwrap_or_else(|| PlayerType::Other),
                    filter 
                }
            }
        };

        if set_as_default {
            _ = config.set_default_config();
        }

        Ok(config)
    }

    fn set_default_config(&self) -> Result<(), std::io::Error> {
        let mut user_config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("player-cli");

        if !user_config_dir.exists() {
            fs::create_dir(&user_config_dir)?;
        }

        user_config_dir = user_config_dir.join("config.json");
        let json = serde_json::to_string(self)?;
        fs::write(user_config_dir, json)?;
        Ok(())
    }

    fn get_default_config() -> Result<Self, std::io::Error> {
        let user_config_file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("player-cli")
            .join("config.json");

        let json_string = fs::read_to_string(user_config_file_path)?;
        let a: Configs = serde_json::from_str(&json_string)?;
        Ok(a)
    }
}
