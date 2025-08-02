use git_url_parse::GitUrl;
use git2::Repository;
use std::path::{Path, PathBuf};

fn get_path(info: &GitUrl, backup_path: &String) -> PathBuf {
    Path::new("")
        .join(backup_path)
        .join(info.host.as_deref().unwrap())
        .join(info.owner.as_deref().unwrap())
        .join(info.name.as_str())
}

pub fn init_or_open_repo(info: &GitUrl, backup_path: &String) -> Result<Repository, String> {
    let repo_path = get_path(info, backup_path);
    if repo_path.is_dir() {
        match Repository::open_bare(repo_path) {
            Ok(r) => return Ok(r),
            Err(e) => return Err(e.to_string()),
        }
    }
    match Repository::init_bare(repo_path) {
        Ok(r) => return Ok(r),
        Err(e) => return Err(e.to_string()),
    }
}
