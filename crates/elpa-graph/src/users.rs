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
