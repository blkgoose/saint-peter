use std::{env, fs::File, process::Command, thread::sleep, time::Duration};

use std::io::Write;

use super::{Error, Result};
use crate::config::Config;

pub fn handle(matches: &clap::ArgMatches, conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or(Error::CannotGetConfig("key_name".to_owned()))?;

    match conf.keys.get(name) {
        Some(user) => {
            Command::new("git")
                .args(["config", "--global", "user.email", &user.git_email])
                .spawn()?;

            sleep(Duration::from_millis(100));

            Command::new("git")
                .args(["config", "--global", "user.name", &user.git_name])
                .spawn()?;

            let mut f = File::create(format!(
                "{}/.ssh/SAINT_PETER_GIT_KEY",
                env::var("HOME").expect("HOME variable should be set")
            ))?;

            write!(f, "{}", user.private_key)
                .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
        }
        None => Err("ERROR: cannot find specified user")?,
    }
}
