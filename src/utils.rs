use std::path::Path;
use colored::Colorize;

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
        "    {} <{}>      {}",
        "-f".bold().magenta(),
        "EXTENSION".dimmed(),
        "Only show files with extension (default: All popular video file extensions included)".green()
    );
println!(
        "    {}       {}",
        "--set-default".bold().magenta(),
        "Set all other options as default".green()
    );
    println!(
        "    {}, {}          {}",
        "--h".bold().magenta(),
        "-help".bold().magenta(),
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
println!();
    println!(
        "    {}",
        "# Play videos with volume at 70 and speed at 2x and set them default".dimmed()
    );
    println!(
        "    {} {} {} {}",
        "player-cli".cyan(),
        "-v 70".yellow(),
        "-s 2".yellow(),
        "--set-default"
    );
println!();
    println!(
        "    {}",
        "# Play videos with volume at 70 and speed at 2x, set them default and show only mp4 files (filters will not be set as default)".dimmed()
    );
    println!(
        "    {} {} {} {} {}",
        "player-cli".cyan(),
        "-v 70".yellow(),
        "-s 2".yellow(),
        "-f mp4",
        "--set-default"
    );

}

pub fn print_files(files: &Vec<String>) {
    for (index, file) in files.iter().enumerate() {
        let subfolder_count = count_slashes(file) - 1;
        let name= Path::new(file).file_name().unwrap().to_str().unwrap();
        println!("{} | {}{}",format_index(index).white(), "/__".repeat(subfolder_count) , name.bold().color(color_for_level(subfolder_count)));
    }
}

pub fn count_slashes(s: &str) -> usize {
        s.chars().filter(|&c| c == '/').count()
}

// text to Colorize color
fn color_for_level(level: usize) -> &'static str {
    match level {
        0 => "white",
        1 => "green",
        2 => "yellow",
        3 => "blue",
        4 => "magenta",
        5 => "cyan",
        6 => "red",
        _ => match (level - 7) % 7 {
            0 => "bright_white",
            1 => "bright_green",
            2 => "bright_yellow",
            3 => "bright_blue",
            4 => "bright_magenta",
            5 => "bright_cyan",
            _ => "bright_red",
        },
    }
}

// Function to format the index based on the requirement
fn format_index(index: usize) -> String {
    if index < 10 {
        format!(" {} ", index)
    } else if index < 100 {
        format!(" {}", index)
    } else {
        format!("{}", index)
    }
}
