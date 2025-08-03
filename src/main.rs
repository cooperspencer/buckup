mod config;
mod git;
mod helper;
mod hosters;

use std::vec;

use clap::Parser;
use tracing::{error, info};

use config::conf;
use git::local;
use helper::url;
use hosters::{common, forgejo, gitea, github};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(short = 'c', long, required = false)]
    configfile: Option<Vec<String>>,
}

fn process_repo(repo: &common::Repository, config: &conf::Config, token: &str) {
    let info = match url::get_info(repo.clone_url.to_string()) {
        Ok(u) => u,
        Err(e) => {
            error!("could not parse repo url: {:?}", e);
            return;
        }
    };

    info!(source = info.host, "processing repo {}", info.name);

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

fn run_for_provider(
    provider: &dyn common::RepoProvider,
    users: Option<&Vec<String>>,
    config: &conf::Config,
    token: &str,
) {
    if let Some(users) = users {
        for user in users {
            info!("backing up repos for {} via {}", user, provider.name());
            match provider.get_user_repos(user) {
                Ok(repos) => {
                    for repo in &repos {
                        process_repo(repo, config, token);
                    }
                }
                Err(e) => error!("error fetching repos: {:?}", e),
            }
        }
    } else {
        info!("backing up repos for your user via {}", provider.name());
        match provider.get_user_repos("") {
            Ok(repos) => {
                for repo in &repos {
                    process_repo(repo, config, token);
                }
            }
            Err(e) => error!("error fetching repos: {:?}", e),
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
                let provider = github::GitHub::new(token.clone());

                run_for_provider(&provider, github.users.as_ref(), &config, &token);
            }
        }

        if let Some(ref gitea_sources) = config.source.gitea {
            for gitea in gitea_sources {
                let token = gitea.token.clone().unwrap_or_default();
                let url = gitea.url.clone().unwrap_or_default();
                let provider = gitea::Gitea::new(token.clone(), url.clone());

                run_for_provider(&provider, gitea.users.as_ref(), &config, &token);
            }
        }

        if let Some(ref forgejo_sources) = config.source.forgejo {
            for forgejo in forgejo_sources {
                let token = forgejo.token.clone().unwrap_or_default();
                let url = forgejo.url.clone().unwrap_or_default();
                let provider = forgejo::Forgejo::new(token.clone(), url.clone());

                run_for_provider(&provider, forgejo.users.as_ref(), &config, &token);
            }
        }
    }
}
