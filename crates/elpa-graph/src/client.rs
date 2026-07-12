use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

const GRAPH_BASE: &str = "https://graph.microsoft.com/v1.0";
const TOKEN_URL: &str = "https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token";

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GraphPage<T> {
    pub value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GraphClient {
    tenant_id: String,
    client_id: String,
    client_secret: String,
    http: reqwest::Client,
    token: Arc<RwLock<Option<String>>>,
}

impl GraphClient {
    pub fn from_env() -> Result<Self> {
        let tenant_id = std::env::var("ENTRA_TENANT_ID")
            .map_err(|_| anyhow!("ENTRA_TENANT_ID not set"))?;

        // Bring-your-own-token: ENTRA_ACCESS_TOKEN skips the client-credentials
        // flow entirely. The token is used as-is and never refreshed: intended
        // for one-shot runs where the caller manages token lifetime (e.g. an
        // admin portal passing its delegated Graph token).
        if let Ok(token) = std::env::var("ENTRA_ACCESS_TOKEN") {
            if !token.trim().is_empty() {
                return Ok(Self {
                    tenant_id,
                    client_id: String::new(),
                    client_secret: String::new(),
                    http: reqwest::Client::new(),
                    token: Arc::new(RwLock::new(Some(token))),
                });
            }
        }

        let client_id = std::env::var("ENTRA_CLIENT_ID")
            .map_err(|_| anyhow!("ENTRA_CLIENT_ID not set"))?;
        let client_secret = std::env::var("ENTRA_CLIENT_SECRET")
            .map_err(|_| anyhow!("ENTRA_CLIENT_SECRET not set"))?;

        Ok(Self {
            tenant_id,
            client_id,
            client_secret,
            http: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
        })
    }

    async fn acquire_token(&self) -> Result<String> {
        let url = TOKEN_URL.replace("{tenant}", &self.tenant_id);
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("scope", "https://graph.microsoft.com/.default"),
        ];
        let resp: TokenResponse = self
            .http
            .post(&url)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.access_token)
    }

    async fn token(&self) -> Result<String> {
        {
            let guard = self.token.read().await;
            if let Some(t) = guard.as_ref() {
                return Ok(t.clone());
            }
        }
        let t = self.acquire_token().await?;
        *self.token.write().await = Some(t.clone());
        Ok(t)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = if path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", GRAPH_BASE, path)
        };
        debug!("GET {}", url);
        let token = self.token().await?;
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    pub async fn get_all_pages<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        let mut all = vec![];
        let first_url = format!("{}{}", GRAPH_BASE, path);
        let mut url = first_url;

        loop {
            let page: GraphPage<T> = self.get(&url).await?;
            all.extend(page.value);
            match page.next_link {
                Some(link) => url = link,
                None => break,
            }
        }
        Ok(all)
    }
}
