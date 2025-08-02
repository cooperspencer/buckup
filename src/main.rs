mod config;
mod git;
mod helper;
mod hosters;

use std::vec;

use better_tracing;
use clap::Parser;
use tracing::{error, info};

use config::conf;
use git::local;
use helper::url;
use hosters::github;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(short = 'c', long, required = false)]
    configfile: Option<Vec<String>>,
}

fn process_repo(repo: &github::Repository, config: &conf::Config, token: &str) {
    let info = match url::get_info(repo.clone_url.to_string()) {
        Ok(u) => u,
        Err(e) => {
            error!("could not parse repo url: {:?}", e);
            return;
        }
    };

    info!(hoster = info.host, "processing repo {}", info.name);

    if let Some(local_dests) = &config.destination.local {
        for local_destination in local_dests {
            let Some(path) = &local_destination.path else {
                error!("no path set in local destination");
                continue;
            };

            local::clone_or_fetch_repo(&info, &repo.clone_url, path, token);
        }
    }
}

fn main() {
    better_tracing::fmt::init();
    let args = CLI::parse();

    let files = match args.configfile {
        Some(f) => f,
        None => vec!["conf.yml".to_string()],
    };

    let configs = conf::get_config(files);

    for config in configs {
        if let Some(ref github_sources) = config.source.github {
            for github in github_sources {
                let token = github.token.clone().unwrap_or_default();

                if let Some(users) = &github.user {
                    for user in users {
                        info!("backing up repos for {}", user);
                        let repos = match github::get_user_repos(user.clone(), &token) {
                            Ok(r) => r,
                            Err(e) => {
                                error!("error fetching repos: {:?}", e);
                                continue;
                            }
                        };

                        for repo in &repos {
                            process_repo(repo, &config, &token);
                        }
                    }
                } else {
                    info!("backing up repos for your user");
                    let repos = match github::get_user_repos("".to_string(), &token) {
                        Ok(r) => r,
                        Err(e) => {
                            error!("error fetching repos: {:?}", e);
                            continue;
                        }
                    };

                    for repo in &repos {
                        process_repo(repo, &config, &token);
                    }
                }
            }
        }
    }
}
