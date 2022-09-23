use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    users: HashMap<String, User>,
    path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    git_name: String,
    git_email: String,
    public_key: String,
    private_key: String,
}

impl Config {
    fn open(path: PathBuf) -> Self {
        match File::open(path.clone()) {
            Ok(f) => Self {
                users: serde_json::from_reader(f).expect("should be deserializable"),
                path,
            },
            Err(_) => Self {
                users: HashMap::new(),
                path,
            },
        }
    }

    fn save(&self) -> () {
        let parent_folder = self.path.parent().unwrap();
        create_dir_all(parent_folder).unwrap();

        let f = File::create(self.path.clone()).expect("file should be available");
        serde_json::to_writer_pretty(f, &self.users).expect("config should be serializable")
    }
}

/// example usage
/// peter add-existing blkgoose --file /home/alessio/.ssh/blkgoose
/// peter use blkgoose
/// peter get-pub blkgoose
fn main() {
    let key_arg = Arg::with_name("key_name").required(true).index(1);

        .version("0.0.1 dev")
    let arguments = App::new("saint-peter")
        .author("Alessio Biancone <alebian1996@gmail.com>")
        .bin_name("saint-peter")
        .subcommand_required(true)
        .subcommand(
            clap::command!("add-existing")
                .arg(key_arg.clone())
                .arg(clap::arg!(--"name" <NAME>).required(true))
                .arg(clap::arg!(--"email" <EMAIL>).required(true))
                .arg(
                    clap::arg!(--"file" <PATH>)
                        .required(true)
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(clap::command!("use").arg(key_arg.clone()))
        .subcommand(clap::command!("get-pub").arg(key_arg.clone()))
        .get_matches();

    let conf_path = format!(
        "{}/.config/saint-peter.json",
        env::var("HOME").expect("HOME variable should be set"),
    );

    let db_path: PathBuf = format!("{}", conf_path).into();

    let mut conf: Config = Config::open(db_path.clone());

    match arguments.subcommand() {
        Some(("add-existing", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();
            let git_name: &String = matches.get_one("name").unwrap();
            let git_email: &String = matches.get_one("email").unwrap();
            let path: &PathBuf = matches.get_one("file").unwrap();

            let pub_file = read_to_string(format!("{}.pub", path.to_string_lossy()));
            let prv_file = read_to_string(path.clone());

            if let (Ok(public_key), Ok(private_key)) = (pub_file, prv_file) {
                conf.users.insert(
                    name.clone(),
                    User {
                        git_name: git_name.clone(),
                        git_email: git_email.clone(),
                        public_key,
                        private_key,
                    },
                );

                conf.save()
            } else {
                println!("ERROR: cannot read input files");
                exit(1)
            }
        }
        Some(("use", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();

            match conf.users.get(name) {
                Some(user) => {
                    Command::new("git")
                        .args(["config", "--global", "user.email", &user.git_email])
                        .spawn()
                        .unwrap();

                    sleep(Duration::from_millis(100));

                    Command::new("git")
                        .args(["config", "--global", "user.name", &user.git_name])
                        .spawn()
                        .unwrap();

                    let mut f = File::create(format!(
                        "{}/.ssh/SAINT_PETER_GIT_KEY",
                        env::var("HOME").expect("HOME variable should be set")
                    ))
                    .unwrap();

                    write!(f, "{}", user.private_key).unwrap();
                }
                None => {
                    println!("ERROR: cannot find specified user");
                    exit(1)
                }
            }
        }
        Some(("get-pub", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();
            match conf.users.get(name) {
                Some(user) => {
                    println!("{}", user.public_key);
                }
                None => {
                    println!("ERROR: cannot find specified user");
                    exit(1)
                }
            }
        }
        _ => unreachable!("what are you doing here?.."),
    };
}
