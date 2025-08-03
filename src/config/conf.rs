use serde::Deserialize;
use serde_derive::Serialize;
use serde_yaml;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub source: Source,
    pub destination: Destination,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Source {
    pub github: Option<Vec<Hoster>>,
    pub gitea: Option<Vec<Hoster>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Destination {
    pub local: Option<Vec<Local>>,
    pub gitea: Option<Vec<Hoster>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Local {
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Hoster {
    pub url: Option<String>,
    pub users: Option<Vec<String>>,
    pub org: Option<Vec<String>>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssh: Option<bool>,
    pub ssh_key: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub exclude_orgs: Option<Vec<String>>,
    pub include_orgs: Option<Vec<String>>,
    pub wiki: Option<bool>,
    pub starred: Option<bool>,
    pub bare: Option<bool>,
}

pub fn get_config(files: Vec<String>) -> Vec<Config> {
    let mut confs: Vec<Config> = Vec::new();
    for file in files {
        if !file.is_empty() {
            let f = std::fs::File::open(&file).expect("file doesn't exist!");

            for document in serde_yaml::Deserializer::from_reader(f) {
                let config = Config::deserialize(document).expect("can't read config!");
                confs.push(config);
            }
        }
    }

    confs
}
