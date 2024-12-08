use color_eyre::eyre::Ok;
use color_eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitignoreTemplate {
    pub name: String,
    pub source: String,
}

pub struct GitHubGitignoreClient {
    client: Client,
    cache_dir: PathBuf,
}

impl GitHubGitignoreClient {
    pub fn new() -> Result<Self> {
        let cache_dir = home::home_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find home directory"))?
            .join(".gitno");

        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            client: Client::new(),
            cache_dir,
        })
    }

    pub async fn fetch_gitignore_templates(&self) -> Result<Vec<GitignoreTemplate>> {
        let url = "https://api.github.com/gitignore/templates";
        let templates: Vec<String> = self
            .client
            .get(url)
            .header("user-Agent", "gitno")
            .send()
            .await?
            .json()
            .await?;

        Ok(templates
            .into_iter()
            .map(|name| GitignoreTemplate {
                name,
                source: String::new(),
            })
            .collect())
    }

    pub async fn fetch_template_content(&self, template_name: &str) -> Result<String> {
        let cache_path = self.cache_dir.join(format!("{}.gitignore", template_name));

        // Check cache first
        if cache_path.exists() {
            return Ok(fs::read_to_string(&cache_path).await?);
        }

        let url = format!(
            "https://api.github.com/gitignore/templates/{}",
            template_name
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "gitno")
            .send()
            .await?;

        let template: serde_json::Value = response.json().await?;
        let content = template["source"].as_str().unwrap_or("").to_string();

        // Cache the template
        let mut file = File::create(&cache_path).await?;
        file.write_all(content.as_bytes()).await?;

        Ok(content)
    }
}
