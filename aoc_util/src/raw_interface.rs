use std::path::PathBuf;

use reqwest::header::COOKIE;

use crate::Error;

pub fn download_input(year: u16, day: u8) -> Result<String, Error> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    // TODO load session cookie from a config file
    let session_cookie = get_session_cookie()?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(COOKIE, format!("session={}", session_cookie.cookie))
        .send()?;
    if !response.status().is_success() {
        return Err(Error::InvalidSessionCookie(session_cookie.source));
    }

    Ok(response.text()?)
}

#[derive(Clone, Debug)]
struct SessionCookie {
    cookie: String,
    source: PathBuf,
}

fn get_session_cookie() -> Result<SessionCookie, Error> {
    let source = dotenvy::dotenv()?;
    let cookie = match dotenvy::var("COOKIE") {
        Ok(c) => c,
        Err(dotenvy::Error::EnvVar(std::env::VarError::NotPresent)) => {
            return Err(Error::NoSessionCookie)
        }
        Err(e) => return Err(e.into()),
    };

    Ok(SessionCookie { cookie, source })
}

#[derive(Clone, Debug)]
pub(crate) struct FolderWalker {
    current_path: Option<PathBuf>,
}

impl FolderWalker {
    pub(crate) fn new_current_dir() -> Result<Self, Error> {
        Ok(Self {
            current_path: Some(std::env::current_dir()?),
        })
    }
}

impl Iterator for FolderWalker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_path {
            Some(ref mut path) => {
                let ret = path.clone();
                if !path.pop() {
                    self.current_path = None;
                }
                Some(ret)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_walker_test() {
        let mut walker = FolderWalker {
            current_path: Some("/a/b/c/d/e".into()),
        };
        assert_eq!(walker.next(), Some("/a/b/c/d/e".into()));
        assert_eq!(walker.next(), Some("/a/b/c/d".into()));
        assert_eq!(walker.next(), Some("/a/b/c".into()));
        assert_eq!(walker.next(), Some("/a/b".into()));
        assert_eq!(walker.next(), Some("/a".into()));
        assert_eq!(walker.next(), Some("/".into()));
        assert_eq!(walker.next(), None);
        assert_eq!(walker.next(), None);
    }
}
