use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub ssh_url: String,
    pub clone_url: String,
    pub has_wiki: bool,
    pub default_branch: String,
}

pub trait RepoProvider {
    fn get_user_repos(&self, user: &str) -> Result<Vec<Repository>, String>;
    fn name(&self) -> &str; // for logging
}
