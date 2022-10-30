use std::{env, path::PathBuf};

use clap::{App, Arg};
use config::Config;

mod config;
mod subcommands;
mod utils;

fn main() {
    let key_arg = Arg::with_name("key_name").required(true).index(1);

    let arguments = App::new("saint-peter")
        .version(clap::crate_version!())
        .author("Alessio Biancone <alebian1996@gmail.com>")
        .bin_name("saint-peter")
        .arg(clap::arg!(--"conf" [CONF])
             .default_value("~/.config/saint-peter.json")
             .value_parser(clap::value_parser!(std::path::PathBuf))
             .help("use a different configuration")
             )
        .subcommand_required(true)
        .subcommand(
            clap::command!("add")
                .arg(key_arg.clone())
                .arg(clap::arg!(--"name" <NAME>).required(true))
                .arg(clap::arg!(--"email" <EMAIL>).required(true))
                .about("add an existing key to the keyring"),
        )
        .subcommand(
            clap::command!("add-existing")
                .arg(key_arg.clone())
                .arg(clap::arg!(--"name" <NAME>).required(true))
                .arg(clap::arg!(--"email" <EMAIL>).required(true))
                .arg(
                    clap::arg!(--"file" <PATH>)
                        .required(true)
                        .value_parser(clap::value_parser!(std::path::PathBuf))
                        .help("Key file where to get pub and pvt key from [no extention, pub key will be taken from '.pub' file]"),
                ).about("add an existing key to the keyring"),
        )
        .subcommand(clap::command!("use")
                    .arg(key_arg.clone())
                    .arg(clap::arg!(--"output-file" [FILE])
                         .default_value("~/.ssh/SAINT_PETER_GIT_KEY")
                         .value_parser(clap::value_parser!(std::path::PathBuf))
                         .help("set where to put the ssh key file")
                         )
                    .about("Switch to a differenct profile"))
        .subcommand(clap::command!("get-pub")
                    .arg(key_arg.clone())
                    .about("Outputs the pub key to later be copied and added to apps that requires it [e.g. github ssh key]"))
        .subcommand(clap::command!("delete")
                    .arg(key_arg.clone())
                    .about("Delete profile"))
        .get_matches();

    let conf_path: &PathBuf = arguments.get_one::<PathBuf>("conf").unwrap();

    let conf: Config = Config::open(conf_path.clone());

    match subcommands::handle(arguments.subcommand(), conf) {
        Err(s) => eprintln!("ERROR: {}", s),
        Ok(()) => (),
    }
}
