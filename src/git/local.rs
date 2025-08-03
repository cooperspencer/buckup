use git_url_parse::GitUrl;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::{Path, PathBuf};
use tracing::{error, info};

fn get_path(info: &GitUrl, backup_path: &str) -> PathBuf {
    Path::new(backup_path)
        .join(info.host.as_deref().unwrap_or("unknown"))
        .join(info.owner.as_deref().unwrap_or("unknown"))
        .join(info.name.as_str())
}

pub fn init_or_open_repo(info: &GitUrl, backup_path: &str) -> Result<Repository, String> {
    let repo_path = get_path(info, backup_path);
    if repo_path.is_dir() {
        Repository::open_bare(repo_path).map_err(|e| e.to_string())
    } else {
        Repository::init_bare(repo_path).map_err(|e| e.to_string())
    }
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

/// Initializes, adds remote (if needed), and fetches the repo
pub fn clone_or_fetch_repo(info: &GitUrl, remote_url: &str, backup_path: &str, token: &str) {
    let hoster = info.host.clone().unwrap_or_else(|| "unknown".to_string());

    let repository = match init_or_open_repo(info, backup_path) {
        Ok(r) => r,
        Err(e) => {
            error!(source = hoster, "failed to init/open repo: {}", e);
            return;
        }
    };

    let mut remote = match repository.find_remote("origin") {
        Ok(r) => r,
        Err(_) => {
            info!(source = hoster, "adding remote");
            match repository.remote("origin", remote_url) {
                Ok(r) => r,
                Err(e) => {
                    error!(source = hoster, "failed to add remote: {}", e.message());
                    return;
                }
            }
        }
    };

    info!(source = hoster, "fetching changes");

    let mut fetch_opts = fetch_options_with_token(token);
    match remote.fetch(&["+refs/*:refs/*"], Some(&mut fetch_opts), None) {
        Ok(_) => info!(source = hoster, "successfully fetched changes"),
        Err(e) => error!(
            source = hoster,
            path = ?repository.path(),
            "fetch failed: {:?}",
            e
        ),
    }
}
