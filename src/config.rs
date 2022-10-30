use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub keys: HashMap<String, Key>,
    path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Key {
    pub git_name: String,
    pub git_email: String,
    pub public_key: String,
    pub private_key: String,
}

impl Config {
    pub fn open(path: PathBuf) -> Self {
        let shellpath: PathBuf = if path.starts_with("~") {
            path.into_os_string()
                .into_string()
                .unwrap()
                .replace("~", env::var("HOME").unwrap().as_str())
                .into()
        } else {
            path
        };

        match File::open(shellpath.clone()) {
            Ok(f) => Self {
                keys: serde_json::from_reader(f).expect("should be deserializable"),
                path: shellpath,
            },
            Err(_) => Self {
                keys: HashMap::new(),
                path: shellpath,
            },
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let parent_folder = self.path.parent().unwrap();
        create_dir_all(parent_folder)?;

        let f = File::create(self.path.clone())?;

        serde_json::to_writer_pretty(f, &self.keys)
            .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
    }
}
