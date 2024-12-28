use std::{env, ffi::OsStr, fs, io::Error, path::Path};
use colored::*;

#[derive(Debug)]
pub struct Configs {
    pub volume_lvl: Option<f32>,
    pub speed: Option<f32>,
}

#[derive(Debug)]
pub enum ConfigError {
    VolumeTypeMismatch,
    SpeedTypeMismatch,
    HelpAsked,
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
                    }
                }
                None => break None,
            }
        };

        match arg_parsed {
            Some(er) => return Err(er),
            None => {}
        }

        Ok(Configs {
            volume_lvl: volume,
            speed,
        })
    }
}

pub fn visit_dirs(dir: &Path, files: &mut Vec<String>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                let name = path.as_path().to_string_lossy().to_string();
                let ext = path
                    .as_path()
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("")
                    .to_lowercase();
                if ["mp4", "mkv", "avi", "wmv", "flv", "mov", "webm"].contains(&ext.as_str()) {
                    files.push(name);
                }
            }
        }
    }
    Ok(())
}


pub fn print_help() {
    println!(
        "{}: {}",
        "player-cli".bold().green(),
        "A simple tool to play videos from the current directory.".yellow()
    );
    println!();
    println!("{}:", "USAGE".bold().blue());
    println!("    {}", "player-cli [OPTIONS]".cyan());
    println!();
    println!("{}:", "OPTIONS".bold().blue());
    println!(
        "    {} <{}>         {}",
        "-v".bold().magenta(),
        "VOLUME".dimmed(),
        "Set the volume level (default: 100)".green()
    );
    println!(
        "    {} <{}>          {}",
        "-s".bold().magenta(),
        "SPEED".dimmed(),
        "Set the playback speed (default: 1.0)".green()
    );
    println!(
        "    {}, {}          {}",
        "-h".bold().magenta(),
        "--help".bold().magenta(),
        "Print this help message and exit".green()
    );
    println!();
    println!("{}:", "EXAMPLES".bold().blue());
    println!("    {}", "# Play videos with default settings".dimmed());
    println!("    {}", "player-cli".cyan());
    println!();
    println!("    {}", "# Play videos with volume set to 50".dimmed());
    println!("    {} {}", "player-cli".cyan(), "-v 50".yellow());
    println!();
    println!("    {}", "# Play videos at 1.5x speed".dimmed());
    println!("    {} {}", "player-cli".cyan(), "-s 1.5".yellow());
    println!();
    println!(
        "    {}",
        "# Play videos with volume at 70 and speed at 2x".dimmed()
    );
    println!(
        "    {} {} {}",
        "player-cli".cyan(),
        "-v 70".yellow(),
        "-s 2".yellow()
    );
}
