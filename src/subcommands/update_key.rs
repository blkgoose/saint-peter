use std::fs::File;

use super::{Error, Result};
use crate::config::Config;
use crate::utils;

use std::io::Write;
use std::path::PathBuf;

pub fn handle(matches: &clap::ArgMatches, conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or(Error::CannotGetConfig("key_name".to_owned()))?;

    let output_path: PathBuf = matches
        .get_one::<PathBuf>("output-file")
        .ok_or_else(|| Error::CannotGetConfig("output-file".to_owned()))?
        .clone();

    match conf.keys.get(name) {
        Some(user) => {
            let mut f = File::create(utils::shellpath(output_path))?;

            write!(f, "{}", user.private_key)
                .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
        }
        None => Err("cannot find specified user")?,
    }
}
