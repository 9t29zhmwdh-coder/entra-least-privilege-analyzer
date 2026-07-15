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
    graph_base: String,
    token_url: String,
}

impl GraphClient {
    /// Client-credentials flow: acquires and caches an app-only token on
    /// first use.
    pub fn new(
        tenant_id: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            http: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
            graph_base: GRAPH_BASE.to_string(),
            token_url: TOKEN_URL.to_string(),
        }
    }

    /// Bring-your-own-token: skips the client-credentials flow entirely. The
    /// token is used as-is and never refreshed, intended for one-shot runs
    /// where the caller manages token lifetime (e.g. an admin portal passing
    /// its delegated Graph token).
    pub fn from_token(tenant_id: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: String::new(),
            client_secret: String::new(),
            http: reqwest::Client::new(),
            token: Arc::new(RwLock::new(Some(token.into()))),
            graph_base: GRAPH_BASE.to_string(),
            token_url: TOKEN_URL.to_string(),
        }
    }

    /// Overrides the Graph API base URL (default: the global commercial
    /// cloud endpoint, `https://graph.microsoft.com/v1.0`). Useful for
    /// national cloud variants (e.g. `https://graph.microsoft.us`) or for
    /// pointing at a mock server in tests.
    pub fn with_graph_base(mut self, base: impl Into<String>) -> Self {
        self.graph_base = base.into();
        self
    }

    /// Overrides the OAuth token endpoint. `{tenant}` is substituted with
    /// this client's tenant id, same as the default.
    pub fn with_token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = url.into();
        self
    }

    pub fn from_env() -> Result<Self> {
        let tenant_id = std::env::var("ENTRA_TENANT_ID")
            .map_err(|_| anyhow!("ENTRA_TENANT_ID not set"))?;

        // Bring-your-own-token: ENTRA_ACCESS_TOKEN skips the client-credentials
        // flow entirely. See `from_token` above.
        if let Ok(token) = std::env::var("ENTRA_ACCESS_TOKEN") {
            if !token.trim().is_empty() {
                return Ok(Self::from_token(tenant_id, token));
            }
        }

        let client_id = std::env::var("ENTRA_CLIENT_ID")
            .map_err(|_| anyhow!("ENTRA_CLIENT_ID not set"))?;
        let client_secret = std::env::var("ENTRA_CLIENT_SECRET")
            .map_err(|_| anyhow!("ENTRA_CLIENT_SECRET not set"))?;

        Ok(Self::new(tenant_id, client_id, client_secret))
    }

    async fn acquire_token(&self) -> Result<String> {
        let url = self.token_url.replace("{tenant}", &self.tenant_id);
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
        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", self.graph_base, path)
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
        let first_url = format!("{}{}", self.graph_base, path);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Widget {
        id: String,
    }

    #[tokio::test]
    async fn from_token_skips_token_acquisition_and_authenticates_directly() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/widgets"))
            .and(header("authorization", "Bearer byot-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [{"id": "1"}],
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "byot-token").with_graph_base(server.uri());

        let widgets: Vec<Widget> = client.get_all_pages("/widgets").await.unwrap();

        assert_eq!(widgets, vec![Widget { id: "1".to_string() }]);
    }

    #[tokio::test]
    async fn client_credentials_flow_acquires_and_reuses_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/tenant/oauth2/v2.0/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "cc-token",
                "token_type": "Bearer",
                "expires_in": 3600,
            })))
            .expect(1) // token must be cached, not re-acquired per request
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/widgets"))
            .and(header("authorization", "Bearer cc-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": "1"})))
            .mount(&server)
            .await;

        let client = GraphClient::new("tenant", "client-id", "client-secret")
            .with_graph_base(server.uri())
            .with_token_url(format!("{}/{{tenant}}/oauth2/v2.0/token", server.uri()));

        let _first: Widget = client.get("/widgets").await.unwrap();
        let _second: Widget = client.get("/widgets").await.unwrap();
    }

    #[tokio::test]
    async fn get_all_pages_follows_odata_next_link_across_pages() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/widgets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [{"id": "1"}, {"id": "2"}],
                "@odata.nextLink": format!("{}/widgets/page2", server.uri()),
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/widgets/page2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [{"id": "3"}],
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let widgets: Vec<Widget> = client.get_all_pages("/widgets").await.unwrap();

        assert_eq!(widgets.len(), 3);
        assert_eq!(widgets[2].id, "3");
    }

    #[tokio::test]
    async fn non_success_status_surfaces_as_an_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/widgets"))
            .respond_with(
                ResponseTemplate::new(429).insert_header("Retry-After", "2"),
            )
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let result: Result<Vec<Widget>> = client.get_all_pages("/widgets").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn absolute_url_is_used_as_is_without_prepending_graph_base() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/already-absolute"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": "x"})))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t")
            .with_graph_base("http://unused.invalid");
        let widget: Widget = client
            .get(&format!("{}/already-absolute", server.uri()))
            .await
            .unwrap();

        assert_eq!(widget.id, "x");
    }
}
