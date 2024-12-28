use ayan_player_cli::{visit_dirs, Configs, print_help};
use std::process::{self, Command};
use std::{env, io::stdin, path::Path};

fn main() {
    let args = env::args();
    let configs = Configs::get_config_from_args(args).unwrap_or_else(|e| match e {
        ayan_player_cli::ConfigError::HelpAsked => {
            print_help();
            process::exit(1);
        }
        _ => {
            println!("Problem parsing arguments: {:?}", e);
            process::exit(1);
        }
    });

    loop {
        let mut files = Vec::new();
        visit_dirs(Path::new("."), &mut files).unwrap_or_else(|e| {
            println!("Problem parsing arguments: {:?}", e);
            process::exit(1);
        });

        if files.is_empty() {
            println!("No video found in directory");
            process::exit(1);
        }
        println!("\n");
        for (i, file) in files.iter().enumerate() {
            println!("{i} {file}");
        }
        println!("\n");

        println!("Enter index to play: ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap_or_else(|e| {
            println!("Error parsing: {e:?}");
            process::exit(1);
        });

        let index_to_play = buffer.trim().parse::<usize>().unwrap_or_else(|e| {
            println!("Error playing: {e:?}");
            process::exit(1);
        });

        let mut c = Command::new("mpv");
        let file = files.get(index_to_play).unwrap();
        let result = c
            .arg(file)
            .arg(format!("--volume={}", configs.get_volume()))
            .arg(format!("--speed={}", configs.get_speed()))
            .output();

        println!("{result:?}");
    }
}


