mod helper;
mod git;
mod hosters;
mod config;

use std::vec;

use tracing::{ info, error, debug };
use tracing_subscriber;
use colored::*;
use clap::{Parser};

use helper::url;
use git::local;
use hosters::github;
use config::conf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
   #[arg(short='c', long, required = false)]
   configfile: Option<Vec<String>>
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = CLI::parse();

    let files = match args.configfile {
        Some(f) => f,
        None => vec!["conf.yml".to_string()],
    };

    let configs = conf::get_config(files);

    let repos = match github::get_user_repos("cooperspencer".to_string(), "".to_string()) {
        Ok(r) => r,
        Err(e) => panic!("{:?}", e),
    };

        for repo in &repos {
            let info = match url::get_info(repo.clone_url.to_string()) {
                Ok(u) => u,
                Err(e) => panic!("{:?}", e)
            };

            let mut hoster = "".to_string();
            if info.host.is_some() {
                hoster = info.host.clone().unwrap();
            }

            info!(hoster=hoster, "initializing {}.git", info.name);

            let repository = match local::init_or_open_repo(info) {
                Ok(r) => r,
                Err(e) => panic!("{}", e.as_str())
            };


            let mut remote = match repository.find_remote("origin") {
                Ok(r) => r,
                Err(_) => {
                    info!(hoster=hoster, "adding remote");
                    match repository.remote("origin", &repo.clone_url) {
                        Ok(r) => r,
                        Err(e) => panic!("{:?}", e),
                    }
                }
            };
            info!(hoster=hoster, "fetching changes");
            match remote.fetch(&["+refs/*:refs/*"], None, None) {
                Ok(_) => info!(hoster=hoster, "successfully fetched changes"),
                Err(e) => error!(hoster=hoster, "a problem occured while fetching! {:?}", e),
            }

    }
}
