use crate::common::{RepoProvider, Repository};
use ureq;

pub struct Forgejo {
    pub token: String,
    pub url: String,
}

impl Forgejo {
    pub fn new(token: String, url: String) -> Self {
        Forgejo { token, url }
    }

    fn get(&self, user: &str, page: u32) -> Result<String, String> {
        let url = if user.is_empty() {
            format!("{}/api/v1/user/repos?limit=100&page={page}", self.url)
        } else {
            format!(
                "{}/api/v1/users/{user}/repos?limit=100&page={page}",
                self.url,
            )
        };

        let request = ureq::get(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("User-Agent", "buckup/1.0");

        let response = request.call().map_err(|e| e.to_string())?;

        Ok(response.into_body().read_to_string().unwrap())
    }
}

impl RepoProvider for Forgejo {
    fn get_user_repos(&self, user: &str) -> Result<Vec<Repository>, String> {
        let mut repos = Vec::new();
        let mut page = 1;

        loop {
            let body = self.get(user, page)?;
            let parsed: Vec<Repository> = serde_json::from_str(&body).map_err(|e| e.to_string())?;

            if parsed.is_empty() {
                break;
            }

            repos.extend(parsed);
            page += 1;
        }

        Ok(repos)
    }

    fn name(&self) -> &str {
        "Forgejo"
    }
}
