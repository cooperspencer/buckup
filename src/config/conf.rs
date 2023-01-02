use clap::Error;
use serde_yaml;
use serde_derive::{Serialize};
use serde::{Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    source: Source,
    destination: Local
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Source {
    github: Option<Vec<Hoster>>,
    gitea: Option<Vec<Hoster>>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Destination {
    local: Option<Vec<Local>>,
    gitea: Option<Vec<Hoster>>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Local {
    path: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Hoster {
    url: Option<String>,
    user: Option<Vec<String>>,
    org: Option<Vec<String>>,
    token: Option<String>,
    username: Option<String>,
    password: Option<String>,
    ssh: Option<bool>,
    ssh_key: Option<String>,
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    exclude_orgs: Option<Vec<String>>,
    include_orgs: Option<Vec<String>>,
    wiki: Option<bool>,
    starred: Option<bool>
}

pub fn get_config(files: Vec<String>) -> Vec<Config> {
    let mut confs: Vec<Config> = Vec::new();
    for file in files {
        if file != "" {
            let f = std::fs::File::open(&file).expect("file doesn't exist!");

            for document in serde_yaml::Deserializer::from_reader(f) {
                let config = Config::deserialize(document).expect("can't read config!");
                confs.push(config);
            }
        }
    }

    confs
}