use super::{Error, Result};
use crate::config::Config;

pub fn handle(matches: &clap::ArgMatches, conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or(Error::CannotGetConfig("key_name".to_owned()))?;

    match conf.keys.get(name) {
        Some(user) => Ok(println!("{}", user.public_key)),
        None => Err("cannot find specified user")?,
    }
}
