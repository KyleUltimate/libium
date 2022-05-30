pub mod config;
pub mod file_picker;
pub mod misc;
pub mod mods;
pub mod game;
pub mod java;
pub mod launcher;
pub mod modpacks;

lazy_static::lazy_static! {
    pub static ref HOME: std::path::PathBuf = home::home_dir().expect("Could not get user's home directory");
}
