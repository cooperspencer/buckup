mod helper;
mod git;
mod hosters;

use tracing::{ info, error, debug };
use tracing_subscriber;

use helper::url;
use git::local;
use hosters::github;

fn main() {
    tracing_subscriber::fmt::init();

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
        remote.fetch(&["+refs/*:refs/*"], None, None).unwrap();
    }

}
