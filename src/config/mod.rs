use std::env;
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub github_token: Option<String>,
    pub api_url: String,
    pub repos: Vec<String>,
    pub orgs: Vec<String>,
    pub users: Vec<String>,
    pub listen_addr: String,
    pub scrape_interval_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env if available
        dotenvy::dotenv().ok();
        
        let github_token = env::var("GITHUB_TOKEN").ok();
        let api_url = env::var("GITHUB_API_URL")
            .unwrap_or_else(|_| "https://api.github.com".to_string());

        // Parse comma-separated lists
        let repos = parse_list(env::var("REPOS").unwrap_or_default());
        let orgs = parse_list(env::var("ORGS").unwrap_or_default());
        let users = parse_list(env::var("USERS").unwrap_or_default());

        // Default listen address
        let listen_addr =
            env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:9100".to_string());

        let scrape_interval_seconds = env::var("SCRAPE_INTERVAL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(300); // default every 5 minutes

        Ok(Self {
            github_token,
            api_url,
            repos,
            orgs,
            users,
            listen_addr,
            scrape_interval_seconds,
        })
    }
}

fn parse_list(value: String) -> Vec<String> {
    value
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}