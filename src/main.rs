use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, read_to_string, File},
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use clap::{App, Arg};
use osshkeys::{KeyPair, KeyType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    keys: HashMap<String, Key>,
    path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct Key {
    git_name: String,
    git_email: String,
    public_key: String,
    private_key: String,
}

impl Config {
    fn open(path: PathBuf) -> Self {
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

    fn save(&self) -> () {
        let parent_folder = self.path.parent().unwrap();
        create_dir_all(parent_folder).unwrap();

        let f = File::create(self.path.clone()).expect("file should be available");
        serde_json::to_writer_pretty(f, &self.keys).expect("config should be serializable")
    }
}

/// example usage
/// peter add-existing blkgoose --file /home/alessio/.ssh/blkgoose
/// peter use blkgoose
/// peter get-pub blkgoose
fn main() {
    let key_arg = Arg::with_name("key_name").required(true).index(1);

    let arguments = App::new("saint-peter")
        .version("0.0.3")
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
                    .about("Switch to a differenct profile"))
        .subcommand(clap::command!("get-pub")
                    .arg(key_arg.clone())
                    .about("Outputs the pub key to later be copied and added to apps that requires it [e.g. github ssh key]"))
        .subcommand(clap::command!("delete")
                    .arg(key_arg.clone())
                    .about("Delete profile"))
        .get_matches();

    let conf_path: &PathBuf = arguments.get_one::<PathBuf>("conf").unwrap();


    let mut conf: Config = Config::open(conf_path.clone());

    match arguments.subcommand() {
        Some(("add", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();
            let git_name: &String = matches.get_one("name").unwrap();
            let git_email: &String = matches.get_one("email").unwrap();

            let password: Option<String> = {
                print!("Enter passphrase (empty for no passphrase): ");

                let mut s = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut s).unwrap();

                s.pop(); // remove "\n"

                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            };

            if conf.keys.contains_key(name) {
                eprint!(
                    "ERROR: keyname exist already, delete it first if you want to overwrite it"
                );
                exit(1);
            }

            let keypair = KeyPair::generate(KeyType::ED25519, 256).unwrap();

            let public_key = keypair.serialize_publickey().unwrap();
            let private_key = keypair
                .serialize_openssh(password.as_deref(), osshkeys::cipher::Cipher::Aes256_Ctr)
                .unwrap();

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
        }
        Some(("add-existing", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();
            let git_name: &String = matches.get_one("name").unwrap();
            let git_email: &String = matches.get_one("email").unwrap();
            let path: &PathBuf = matches.get_one("file").unwrap();

            if conf.keys.contains_key(name) {
                eprint!("ERROR: keyname exist already, delete it first if you want to overwrite it")
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
                eprintln!("ERROR: cannot read input files");
                exit(1)
            }
        }
        Some(("use", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();

            match conf.keys.get(name) {
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
                    eprintln!("ERROR: cannot find specified user");
                    exit(1)
                }
            }
        }
        Some(("get-pub", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();
            match conf.keys.get(name) {
                Some(user) => {
                    eprintln!("{}", user.public_key);
                }
                None => {
                    eprintln!("ERROR: cannot find specified user");
                    exit(1)
                }
            }
        }
        Some(("delete", matches)) => {
            let name: &String = matches.get_one("key_name").unwrap();

            match conf.keys.get(name) {
                Some(_) => {
                    conf.keys.remove(name);
                    conf.save();
                }
                None => {
                    eprintln!("ERROR: cannot find specified user");
                    exit(1)
                }
            }
        }
        _ => unreachable!("what are you doing here?.."),
    };
}
