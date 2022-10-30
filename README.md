# saint-peter
### git ssh key-store manager

#### Why & How
Having multiple github accounts is not supported easily by the git cli tool,
this tool provides that functionality.

It stores **keyname**, **username**, **email** and **ssh-key**, when switching key it will also
switch the **username** and **email**, so that you won't push using the wrong account.

#### Installation
``` bash
cargo build --release
sudo install target/release/saint-peter /usr/bin/saint-peter
```

Or from crates.io:
``` bash
cargo install saint-peter
```


Use `~/.ssh/SAINT_PETER_GIT_KEY` in your configuration to use saint-peter as key-store.
> NOTE: you can change the above key position using the `--output-file` option. [default: `~/.ssh/SAINT_PETER_GIT_KEY`]
<br>
Here an example:
<br>
inside the file `~/.ssh/config`

``` ssh-config
Host github.com
   HostName github.com
   User git
   IdentityFile ~/.ssh/SAINT_PETER_GIT_KEY
```

#### Usage
##### Generate and add a new key to the store
``` bash
saint-peter add \
    --name <username> \
    --email <email> \
    <keyname>
```

##### Add existing key to saint-peter store
``` bash
saint-peter add-existing \
    --name <username> \
    --email <email> \
    --file ~/.ssh/<ssh_key_name> \
    <keyname>
```
> NOTE: The old file can be deleted as it is stored inside the internal store

##### Use key
``` bash
saint-peter use <keyname>
```

##### Get pubblic key
``` bash
saint-peter get-pub <keyname>
```
> NOTE: Useful in order to add the key to a service like github

##### Delete saved key
``` bash
saint-peter delete <keyname>
```

#### Edit configuration
The configuration is in plain json, if one need to change any of the data, like **username** or **email**,
it can be done easily in the stored json.
> NOTE: position of this file can be changed with the `--config` option [default: `~/.config/saint-peter.json`]
