use std::{env, path::PathBuf};

pub fn shellpath(path: PathBuf) -> PathBuf {
    if path.starts_with("~") {
        path.into_os_string()
            .into_string()
            .unwrap()
            .replace("~", env::var("HOME").unwrap().as_str())
            .into()
    } else {
        path
    }
}
