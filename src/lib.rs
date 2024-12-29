use colored::*;
use std::{ffi::OsStr, fs, io::Error, path::Path};
pub mod player;
pub mod config;

// public functions
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

pub fn print_file(index: usize, file: &str) {
    if file.chars().filter(|c| *c == '/').count() > 1 {
        println!("({}) {}", index.to_string().white(), file.bold().blue());
    } else {
        println!("({}) {}", index.to_string().white(), file.bold().white());
    }
}
