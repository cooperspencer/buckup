use ureq;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
}

fn get(
    sort: String,
    user: &String,
    token: &String,
    what: String,
    page: u32,
) -> Result<String, String> {
    let url = match user.is_empty() {
        true => format!(
            "https://api.github.com/{}/{}?per_page=100&page={}",
            sort, what, page
        )
        .to_owned(),
        false => format!(
            "https://api.github.com/{}/{}/{}?per_page=100&page={}",
            &sort, &user, what, page
        ),
    };

    if token.is_empty() {
        let res = match ureq::get(&url).call() {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        Ok(res.into_body().read_to_string().unwrap())
    } else {
        let res = match ureq::get(&url)
            .header("Authorization", &format!("Bearer {}", token))
            .call()
        {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        Ok(res.into_body().read_to_string().unwrap())
    }
}

pub fn get_user_repos(user: String, token: &String) -> Result<Vec<Repository>, String> {
    let mut repos: Vec<Repository> = vec![];
    let mut page = 1;

    loop {
        let text = match get("users".to_string(), &user, token, "repos".to_string(), page) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string()),
        };

        let r: Vec<Repository> = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        if r.len() > 0 {
            repos.append(&mut r.clone());
        } else {
            break;
        }

        page += 1;
    }
    Ok(repos)
}

pub fn get_repos_authenticated(token: &String) -> Result<Vec<Repository>, String> {
    let mut repos: Vec<Repository> = vec![];
    let mut page = 1;

    loop {
        let text = match get(
            "user".to_string(),
            &"".to_string(),
            token,
            "repos".to_string(),
            page,
        ) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string()),
        };

        let r: Vec<Repository> = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        if r.len() > 0 {
            repos.append(&mut r.clone());
        } else {
            break;
        }

        page += 1;
    }
    Ok(repos)
}
