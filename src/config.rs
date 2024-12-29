use colored::Colorize;
use dirs;
use std::{self, env, fs, path::PathBuf};

use crate::player::{get_player_from_str, PlayerType};

#[derive(Debug)]
pub enum ConfigError {
    VolumeTypeMismatch,
    SpeedTypeMismatch,
    PlayerTypeMismatch,
    HelpAsked,
}

#[derive(Debug)]
pub struct Configs {
    volume_lvl: Option<f32>,
    speed: Option<f32>,
    pub player: PlayerType,
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
        let mut player: PlayerType = PlayerType::Other;
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
                        player = get_player_from_str(&player_arg);
                    } else if arg == "-set-default" {
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

        if matches!(player, PlayerType::Other) {
            println!("\n{}: Video player is not specified, so the default player will be used, hence '-s' and '-v' options will not work", "!Warning".bold().yellow());
            println!(
                "  {} {}",
                "·".bold(),
                "Use '-player' option to specify the player."
                    .bold()
                    .yellow()
            );
        }

        let config = Configs {
            volume_lvl: volume,
            speed,
            player: player,
        };

        if set_as_default {
            _ = config.set_default_config().unwrap_or_else(|e| {
                println!("Error setting default config: {:?}", e);
            });
        }

        Ok(config)
    }

    fn to_json(&self) -> String {
        format!(
            r#"{{
  "volume_lvl": {},
  "speed": {},
  "player": "{}"
}}"#,
            self.get_volume(),
            self.get_speed(),
            self.player
        )
    }

    pub fn set_default_config(&self) -> Result<(), std::io::Error> {
        let mut user_config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("player-cli");

        if !user_config_dir.exists() {
            fs::create_dir(&user_config_dir)?;
        }

        user_config_dir = user_config_dir.join("config.json");
        fs::write(user_config_dir, self.to_json())?;
        Ok(())
    }
}
