use std::io::{stdin, stdout, Write};

use osshkeys::{KeyPair, KeyType};

use super::{Error, Result};
use crate::config::{Config, Key};

pub fn handle(matches: &clap::ArgMatches, mut conf: Config) -> Result<()> {
    let name: &String = matches
        .get_one("key_name")
        .ok_or(Error::CannotGetConfig("key_name".to_owned()))?;

    let git_name: &String = matches
        .get_one("name")
        .ok_or(Error::CannotGetConfig("name".to_owned()))?;

    let git_email: &String = matches
        .get_one("email")
        .ok_or(Error::CannotGetConfig("email".to_owned()))?;

    let password: Option<String> = {
        print!("Enter passphrase (empty for no passphrase): ");

        let mut s = String::new();
        let _ = stdout().flush();
        // TODO: input without display
        stdin().read_line(&mut s).unwrap();

        s.pop(); // remove "\n"

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    };

    if conf.keys.contains_key(name) {
        Err("key_name exists already, delete it first if you want to overwrite it")?
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
