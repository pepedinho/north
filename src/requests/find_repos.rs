use reqwest::Client;
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub full_name: String,
    pub owner: Owner,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub stargazers_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
}

pub struct GitHubClient {
    client: Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        GitHubClient {
            client: Client::new(),
        }
    }

    pub async fn search_repos(&self, query: &str) -> Result<Vec<Repo>, reqwest::Error> {
        let url = format!("{}/search/repositories?q={}", GITHUB_API_BASE, query);
        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "north")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let items = resp["items"].as_array().unwrap();
        for item in items {
            println!("debug {}", item)
        }

        Ok(items
            .iter()
            .map(|item| serde_json::from_value(item.clone()).unwrap())
            .collect())
    }

    pub async fn get_readme(&self, owner: &str, repo: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/repos/{}/{}/readme", GITHUB_API_BASE, owner, repo);
        println!("debug : {}", url);
        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "north")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let content = resp["content"].as_str().unwrap();
        let cleaned_content = content.replace('\n', "").replace('\r', "");
        let decoded = base64::decode(cleaned_content.trim_end()).unwrap();
        let readme = String::from_utf8(decoded).unwrap();
        Ok(readme)
    }
}
