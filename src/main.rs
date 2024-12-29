use ayan_player_cli::player::play;
use ayan_player_cli::{print_file, print_help, visit_dirs, Configs};
use std::iter::Skip;
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

        // gets files list
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
            print_file(i, file);
        }
        println!("\n");

        println!("Enter index to play ('q' to quit): ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap_or_else(|e| {
            println!("Error parsing: {e:?}");
            process::exit(1);
        });

        // quit if 'q'
        if buffer.trim() == "q" {
            process::exit(1);
        }

        let index_to_play = buffer.trim().parse::<usize>().unwrap_or_else(|e| {
            println!("Error playing: {e:?}");
            process::exit(1);
        });

        let result = play(&files[index_to_play], &configs);
        println!("{result:?}");
    }
}
