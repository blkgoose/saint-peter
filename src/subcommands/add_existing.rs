use std::{fs::read_to_string, path::PathBuf};

use super::{Error, Result};
use crate::config::{Config, Key};

pub fn handle(matches: &clap::ArgMatches, mut conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or_else(|| Error::CannotGetConfig("key_name".to_owned()))?;

    let git_name: &String = matches
        .get_one("name")
        .ok_or_else(|| Error::CannotGetConfig("name".to_owned()))?;

    let git_email: &String = matches
        .get_one("email")
        .ok_or_else(|| Error::CannotGetConfig("email".to_owned()))?;

    let path: &PathBuf = matches
        .get_one::<PathBuf>("file")
        .ok_or_else(|| Error::CannotGetConfig("key_name".to_owned()))?;

    if conf.keys.contains_key(name) {
        Err("key_name exists already, delete it first if you want to overwrite it")?
    }

    let pub_file = read_to_string(format!("{}.pub", path.to_string_lossy()));
    let prv_file = read_to_string(path.clone());

    if let (Ok(public_key), Ok(private_key)) = (pub_file, prv_file) {
        conf.keys.insert(
            name.clone(),
            Key {
                git_name: git_name.clone(),
                git_email: git_email.clone(),
                public_key,
                private_key,
            },
        );

        conf.save()
    } else {
        Err("cannot read input files")?
    }
}
