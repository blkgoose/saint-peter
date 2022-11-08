use crate::config::Config;

mod add;
mod add_existing;
mod delete;
mod get_pub;
mod set_current;
mod use_cmd;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, strum_macros::Display)]
pub enum Error {
    CannotGetConfig(String),
}
impl std::error::Error for Error {}

pub fn handle(subcommand: Option<(&str, &clap::ArgMatches)>, conf: Config) -> Result<()> {
    match subcommand {
        Some(("add", matches)) => add::handle(matches, conf),
        Some(("add-existing", matches)) => add_existing::handle(matches, conf),
        Some(("use", matches)) => use_cmd::handle(matches, conf),
        Some(("set-current", matches)) => set_current::handle(matches, conf),
        Some(("get-pub", matches)) => get_pub::handle(matches, conf),
        Some(("delete", matches)) => delete::handle(matches, conf),

        _ => unreachable!("what are you doing here?.."),
    }
}
