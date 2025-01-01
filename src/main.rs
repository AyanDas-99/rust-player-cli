use ayan_player_cli::{
    count_slashes, player::play, print_files, print_help, visit_dirs, ConfigError, Configs,
};
use std::process;
use std::{env, io::stdin, path::Path};

fn main() {
    let args = env::args();
    let configs = Configs::get_config_from_args(args).unwrap_or_else(|e| match e {
        ConfigError::HelpAsked => {
            print_help();
            process::exit(1);
        }
        _ => {
            println!("Problem parsing arguments: {:?}", e);
            process::exit(1);
        }
    });

    println!("Config: {configs:?}");

    loop {
        let mut files = Vec::new();

        // gets files list
        visit_dirs(Path::new("."), &mut files, &configs).unwrap_or_else(|e| {
            println!("Problem parsing arguments: {:?}", e);
            process::exit(1);
        });

        if files.is_empty() {
            println!("No video found in directory");
            process::exit(1);
        }

        // sort files based on subfolder count
        files.sort_by(|a, b| count_slashes(&a).cmp(&count_slashes(&b)));

        println!("\n");

        // print all the files
        print_files(&files);

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
