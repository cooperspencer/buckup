use git2::{ Repository };
use git_url_parse::GitUrl;
use std::path::{Path, PathBuf};

fn get_path(info: GitUrl) -> PathBuf {
    Path::new("").join("backup").join(info.host.unwrap()).join(info.owner.unwrap()).join(info.name)
}

pub fn init_or_open_repo(info: GitUrl) -> Result<Repository, String> {
    let repo_path = get_path(info);
    println!("{:?}", repo_path);
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