use std::{
    io,
    process::{Command, Output},
};

use crate::config::Configs;



#[derive(Debug)]
pub enum PlayerType {
    MPV,
    VLC,
    Other,
}

impl PlayerType {
    pub fn build_command(&self, file: &str, config: &Configs) -> Command {
        let mut cmd = Command::new(match self {
            PlayerType::MPV => format!("mpv"),
            PlayerType::VLC => format!("vlc"),
            PlayerType::Other => format!("xdg-open"),
        });
        cmd.arg(file);
        self.build_volume(&config, &mut cmd);
        self.build_speed(&config, &mut cmd);
        cmd
    }

    fn build_volume(&self, config: &Configs, command: &mut Command) {
        match self {
            PlayerType::MPV => command.arg(format!("--volume={}", config.get_volume())),
            PlayerType::VLC => command.arg(format!(
                "--volume {}",
                (config.get_volume() / 100.0) * 256.0
            )),
            PlayerType::Other => command,
        };
    }
    fn build_speed(&self, config: &Configs, command: &mut Command) {
        match self {
            PlayerType::MPV => command.arg(format!("--speed={}", config.get_speed())),
            PlayerType::VLC => command.arg(format!("--rate {}", config.get_speed())),
            PlayerType::Other => command,
        };
    }
}

pub fn get_player_from_str(config: &str) -> PlayerType {
    let player_str = config.to_lowercase();
    if player_str == "mpv" {
        return PlayerType::MPV;
    } else if player_str == "vlc" {
        return PlayerType::VLC;
    } else {
        return PlayerType::Other;
    }
}

pub fn play(file: &str, config: &Configs) -> Result<Output, io::Error> {
    let mut cmd = config.player.build_command(file, &config);
    println!("Command used: {cmd:?}");
    let result = cmd.output();
    result
}
