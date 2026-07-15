use anyhow::Result;
use serde::Deserialize;

use crate::client::GraphClient;
use elpa_core::models::User;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphUser {
    id: String,
    display_name: Option<String>,
    user_principal_name: Option<String>,
    account_enabled: Option<bool>,
}

impl From<GraphUser> for User {
    fn from(u: GraphUser) -> Self {
        User {
            id: u.id,
            display_name: u.display_name.unwrap_or_default(),
            user_principal_name: u.user_principal_name.unwrap_or_default(),
            account_enabled: u.account_enabled.unwrap_or(true),
        }
    }
}

pub async fn list_users(client: &GraphClient) -> Result<Vec<User>> {
    let raw: Vec<GraphUser> = client
        .get_all_pages("/users?$select=id,displayName,userPrincipalName,accountEnabled")
        .await?;
    Ok(raw.into_iter().map(User::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GraphClient;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn graph_user_maps_missing_optional_fields_to_sane_defaults() {
        let raw = GraphUser {
            id: "u1".to_string(),
            display_name: None,
            user_principal_name: None,
            account_enabled: None,
        };

        let user: User = raw.into();

        assert_eq!(user.id, "u1");
        assert_eq!(user.display_name, "");
        assert_eq!(user.user_principal_name, "");
        // Graph omits accountEnabled very rarely, but when it does the safer
        // default for a privilege-analysis tool is "assume enabled" so a
        // disabled-but-privileged account is never silently hidden.
        assert!(user.account_enabled);
    }

    #[test]
    fn graph_user_preserves_explicit_disabled_flag() {
        let raw = GraphUser {
            id: "u1".to_string(),
            display_name: Some("Alice".to_string()),
            user_principal_name: Some("alice@contoso.com".to_string()),
            account_enabled: Some(false),
        };

        let user: User = raw.into();

        assert!(!user.account_enabled);
    }

    #[tokio::test]
    async fn list_users_fetches_and_maps_all_pages() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [
                    {"id": "1", "displayName": "Alice", "userPrincipalName": "alice@contoso.com", "accountEnabled": true},
                    {"id": "2", "displayName": "Bob", "userPrincipalName": "bob@contoso.com", "accountEnabled": false}
                ]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let users = list_users(&client).await.unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].display_name, "Alice");
        assert!(!users[1].account_enabled);
    }
}
