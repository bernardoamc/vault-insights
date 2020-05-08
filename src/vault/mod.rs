use anyhow::Result;
use serde_json::Value;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use tokio;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub mod project;
use project::Project;

const PROJECT_API_PATH: &'static str = "api/projects";
const PARALLEL_REQUESTS: usize = 8;

pub struct Vault<'a> {
    key: &'a str,
    token: &'a str,
    vault_url: &'a str,
    project_ids: &'a [u32]
}

impl<'a> Vault<'a> {
    pub fn new(key: &'a str, token: &'a str, vault_url: &'a str, project_ids: &'a [u32]) -> Self {
        Self { key, token, vault_url, project_ids }
    }

    #[tokio::main]
    pub async fn fetch_projects(&self) -> Result<Vec<Project>> {      
        let client = Client::new();
        let project_api_url = format!("{}/{}", self.vault_url, PROJECT_API_PATH);

        let mut responses = stream::iter(self.project_ids).map(|project_id| {
            let client = &client;

            let url = format!("{}/{}", project_api_url, project_id);

            async move {
                let response = client
                .get(&url)
                .header(ACCEPT, "application/vnd.api+json")
                .header(CONTENT_TYPE, "application/vnd.api+json")
                .basic_auth(self.key, Some(self.token))
                .send()
                .await?;

                if response.status() == 401 {
                    eprintln!("{}", "Invalid credentials provided, check your configuration file.");
                    std::process::exit(1);
                }
                
                response.text().await
            }
        })
        .buffer_unordered(PARALLEL_REQUESTS);

        let mut result = vec![];

        while let Some(response) = responses.next().await {
            let project = match response {
                Ok(body) => {
                    let mut parsed: Value = serde_json::from_str(&body)?;
                    Project::parse(Some(&mut parsed), self.vault_url)
                },
                Err(_msg) => {
                    println!("Failed to fetch one of the projects...");
                    Project::parse(None, self.vault_url)
                }
            };

            result.push(project);
        }
        
        Ok(result)
    } 
}
