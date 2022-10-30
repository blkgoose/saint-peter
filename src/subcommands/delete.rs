use super::Result;
use crate::config::Config;

pub fn handle(matches: &clap::ArgMatches, mut conf: Config) -> Result<()> {
    let name: &String = matches.get_one("key_name").unwrap();

    match conf.keys.get(name) {
        Some(_) => {
            conf.keys.remove(name);
            conf.save()
        }
        None => Err("ERROR: cannot find specified user")?,
    }
}
