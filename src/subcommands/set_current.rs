use std::{path::PathBuf, process::Command, thread::sleep, time::Duration};

use super::{Error, Result};
use crate::config::Config;

pub fn handle(matches: &clap::ArgMatches, conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or(Error::CannotGetConfig("key_name".to_owned()))?;

    let output_path: String = matches
        .get_one::<PathBuf>("output-file")
        .ok_or_else(|| Error::CannotGetConfig("output-file".to_owned()))?
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|_| Error::CannotGetConfig("output-file".to_owned()))?;

    match conf.keys.get(name) {
        Some(user) => {
            Command::new("git")
                .args(["config", "user.email", &user.git_email])
                .spawn()?;

            sleep(Duration::from_millis(100));

            Command::new("git")
                .args(["config", "user.name", &user.git_name])
                .spawn()?;

            sleep(Duration::from_millis(100));

            Command::new("git")
                .args([
                    "config",
                    "core.sshCommand",
                    format!(
                        "saint-peter update-key --output-file '{}' {}; ssh",
                        output_path, name
                    )
                    .as_ref(),
                ])
                .spawn()?;

            Ok(())
        }
        None => Err("cannot find specified user")?,
    }
}
