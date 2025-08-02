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

use git2::{Cred, FetchOptions, RemoteCallbacks};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(short = 'c', long, required = false)]
    configfile: Option<Vec<String>>,
}

fn fetch_options_with_token(token: &str) -> FetchOptions<'_> {
    let mut callbacks = RemoteCallbacks::new();

    if !token.is_empty() {
        callbacks.credentials(move |_, _, _| Cred::userpass_plaintext("x-access-token", token));
    }

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    fetch_options
}

fn process_repo(repo: &github::Repository, config: &conf::Config, token: &str) {
    let info = match url::get_info(repo.clone_url.to_string()) {
        Ok(u) => u,
        Err(e) => {
            error!("could not parse repo url: {:?}", e);
            return;
        }
    };

    let hoster = info.host.clone().unwrap_or_else(|| "unknown".to_string());

    info!(hoster = hoster, "initializing {}.git", info.name);

    if let Some(local_dests) = &config.destination.local {
        for local_destination in local_dests {
            let Some(path) = &local_destination.path else {
                error!("no path set in local destination");
                continue;
            };

            let repository = match local::init_or_open_repo(&info, &path) {
                Ok(r) => r,
                Err(e) => {
                    error!("could not open repo: {}", e.as_str());
                    continue;
                }
            };

            let mut remote = match repository.find_remote("origin") {
                Ok(r) => r,
                Err(_) => {
                    info!(
                        hoster = hoster,
                        path = ?repository.path(),
                        "adding remote"
                    );
                    match repository.remote("origin", &repo.clone_url) {
                        Ok(r) => r,
                        Err(e) => {
                            error!("failed to add remote: {}", e.message());
                            continue;
                        }
                    }
                }
            };

            info!(hoster = hoster, path = ?repository.path(), "fetching changes");

            let mut fetch_opts = fetch_options_with_token(token);
            match remote.fetch(&["+refs/*:refs/*"], Some(&mut fetch_opts), None) {
                Ok(_) => {
                    info!(
                        hoster = hoster,
                        path = ?repository.path(),
                        "successfully fetched changes"
                    );
                }
                Err(e) => {
                    error!(
                        hoster = hoster,
                        path = ?repository.path(),
                        "a problem occurred while fetching: {:?}",
                        e
                    );
                }
            }
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
