use git_url_parse::GitUrl;

pub fn get_info(url: String) -> Result<GitUrl, String> {
    match GitUrl::parse(&url) {
        Ok(u) => return Ok(u),
        Err(e) => return Err(e.to_string()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https() {
        let url = get_info("https://github.com/cooperspencer".to_string()).unwrap();
        assert_eq!(url.host.unwrap().to_string(), "github.com");
    }
    #[test]
    fn ssh() {
        let url = get_info("git@github.com:cooperspencer/gickup.git".to_string()).unwrap();
        assert_eq!(url.host.unwrap().to_string(), "github.com");
    }
    #[test]
    fn gitea() {
        let url = get_info("https://gitea.com/gitea/awesome-gitea".to_string()).unwrap();
        assert_eq!(url.host.unwrap().to_string(), "gitea.com");
    }
    #[test]
    fn onedev() {
        let url = get_info("https://code.onedev.io/onedev/server".to_string()).unwrap();
        assert_eq!(url.host.unwrap().to_string(), "code.onedev.io");
    }
}
