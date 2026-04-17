use crate::common::{RepoProvider, Repository};
use ureq;

pub struct Gogs {
    pub token: String,
    pub url: String,
}

impl Gogs {
    pub fn new(token: String, url: String) -> Self {
        Gogs { token, url }
    }

    fn get(&self, user: &str) -> Result<String, String> {
        let url = if user.is_empty() {
            format!("{}/api/v1/user/repos", self.url)
        } else {
            format!("{}/api/v1/users/{user}/repos", self.url,)
        };

        let request = ureq::get(&url)
            .header("Authorization", &format!("token {}", self.token))
            .header("User-Agent", "buckup/1.0");

        let response = request.call().map_err(|e| e.to_string())?;

        Ok(response.into_body().read_to_string().unwrap())
    }
}

impl RepoProvider for Gogs {
    fn get_user_repos(&self, user: &str) -> Result<Vec<Repository>, String> {
        let mut repos = Vec::new();

        let body = self.get(user)?;
        let parsed: Vec<Repository> = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        repos.extend(parsed);

        Ok(repos)
    }

    fn name(&self) -> &str {
        "Gogs"
    }
}
