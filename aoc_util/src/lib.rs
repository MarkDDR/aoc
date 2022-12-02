use std::path::PathBuf;

use raw_interface::download_input;
use thiserror::Error;

pub mod raw_interface;

pub fn get_input_string(year: u16, day: u8) -> Result<String, Error> {
    let folder_walker = raw_interface::FolderWalker::new_current_dir()?;

    let input_name = format!("input_{}_{:02}.txt", year, day);
    for file_check in folder_walker.map(|mut p| {
        p.push(&input_name);
        p
    }) {
        if file_check.is_file() {
            // eprintln!("[AOC] Found input at {}", file_check.display());

            let input_text = std::fs::read_to_string(file_check)?;
            return Ok(input_text);
        }
    }

    eprintln!("[AOC] No stored input, downloading");

    // no file, download and store it
    let input = download_input(year, day)?;
    let mut cache_path = std::env::current_dir()?;
    cache_path.push(&input_name);

    eprintln!("[AOC] Stored input at {}", cache_path.display());

    std::fs::write(cache_path, input.as_bytes())?;
    Ok(input)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "session cookie file not found. Make a `.env` file with the cookie stored as `COOKIE=...`"
    )]
    NoSessionCookie,
    #[error("session cookie found at `{0}` is invalid")]
    InvalidSessionCookie(PathBuf),
    #[error("http error `{0}`")]
    HttpError(#[from] reqwest::Error),
    #[error("dotenvy error `{0}`")]
    DotEnvError(#[from] dotenvy::Error),
    #[error("IO error `{0}`")]
    IOError(#[from] std::io::Error),
}
