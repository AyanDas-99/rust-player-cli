use std::{ffi::OsStr, fs, io::Error, path::Path};
pub mod player;
pub mod config;
pub mod utils;


pub use utils::print_help;
pub use utils::print_files;
pub use utils::count_slashes;
pub use config::*;

// public functions
pub fn visit_dirs(dir: &Path, files: &mut Vec<String>, config: &Configs) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files, config)?;
            } else {
                let name = path.as_path().to_string_lossy().to_string();
                let ext = path
                    .as_path()
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("")
                    .to_lowercase();
                match &config.filter {
                    Some(a) => {
                        if a == &ext {
                            files.push(name);
                        }
                    },
                    None => {
                        if ["mp4", "mkv", "avi", "wmv", "flv", "mov", "webm"].contains(&ext.as_str()) {
                            files.push(name);
                        }
                    }
                }


            }
        }
    }
    Ok(())
}


