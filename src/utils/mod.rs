pub mod geometry;
pub mod ui;

use std::env;
use std::path::PathBuf;

pub fn get_cwd() -> PathBuf {
    let cwd = env::current_dir().unwrap_or(PathBuf::from("."));
    let user_dir = env::var_os("USERPROFILE")
        .expect("FATAL ERROR. USERPROFILE environment key missing");

    cwd.canonicalize().unwrap_or(PathBuf::from(user_dir))
}

pub fn get_cwd_as_str() -> String {
    get_cwd().to_string_lossy().to_string()
}