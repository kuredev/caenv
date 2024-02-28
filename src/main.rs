extern crate serde;
use dialoguer::Select;
use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use toml;
use dirs;

#[derive(Debug, Deserialize)]
struct Profile {
    api_key: String,
}

fn main() {
    let home_dir = dirs::home_dir().unwrap();

    let mut file = File::open(home_dir.join(".cloudautomator/credentials")).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let profiles: HashMap<String, Profile> = toml::from_str(&contents).unwrap();
    let profile_names: Vec<String> = profiles.keys().cloned().collect();

    let result = Select::new()
    .with_prompt("What environment variable do you choose?")
    .items(&profile_names)
    .interact()
    .unwrap();

    println!("{:?}", profiles.get(&profile_names[result]).unwrap().api_key);
}
