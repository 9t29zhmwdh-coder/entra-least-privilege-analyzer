use anyhow::Result;
use serde::Deserialize;

use crate::client::GraphClient;
use elpa_core::models::{AssignmentType, RoleAssignment};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphRoleDefinition {
    #[allow(dead_code)]
    id: String,
    display_name: Option<String>,
    #[allow(dead_code)]
    is_built_in: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphPrincipal {
    id: Option<String>,
    user_principal_name: Option<String>,
    #[serde(rename = "@odata.type")]
    odata_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphRoleAssignment {
    #[allow(dead_code)]
    id: String,
    role_definition_id: String,
    principal_id: Option<String>,
    principal: Option<GraphPrincipal>,
    role_definition: Option<GraphRoleDefinition>,
}

pub async fn list_role_assignments(client: &GraphClient) -> Result<Vec<RoleAssignment>> {
    let raw: Vec<GraphRoleAssignment> = client
        .get_all_pages(
            "/roleManagement/directory/roleAssignments?$expand=principal,roleDefinition",
        )
        .await?;

    let assignments = raw
        .into_iter()
        .filter_map(|a| {
            let principal = a.principal.as_ref()?;
            let odata_type = principal.odata_type.as_deref().unwrap_or("");
            if !odata_type.contains("user") {
                return None;
            }
            let user_id = principal.id.clone().unwrap_or_else(|| a.principal_id.clone().unwrap_or_default());
            let upn = principal
                .user_principal_name
                .clone()
                .unwrap_or_else(|| user_id.clone());
            let role_name = a
                .role_definition
                .as_ref()
                .and_then(|r| r.display_name.clone())
                .unwrap_or_else(|| a.role_definition_id.clone());

            Some(RoleAssignment {
                user_id,
                user_principal_name: upn,
                role_id: a.role_definition_id,
                role_name,
                assignment_type: AssignmentType::Direct,
                is_permanent: true,
                expires_at: None,
            })
        })
        .collect();

    Ok(assignments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GraphClient;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn maps_and_filters_role_assignments_to_users_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/roleManagement/directory/roleAssignments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [
                    {
                        "id": "assignment-1",
                        "roleDefinitionId": "role-1",
                        "principalId": "user-1",
                        "principal": {
                            "id": "user-1",
                            "userPrincipalName": "alice@contoso.com",
                            "@odata.type": "#microsoft.graph.user"
                        },
                        "roleDefinition": { "id": "role-1", "displayName": "Global Administrator", "isBuiltIn": true }
                    },
                    {
                        "id": "assignment-2",
                        "roleDefinitionId": "role-2",
                        "principalId": "sp-1",
                        "principal": {
                            "id": "sp-1",
                            "userPrincipalName": null,
                            "@odata.type": "#microsoft.graph.servicePrincipal"
                        },
                        "roleDefinition": { "id": "role-2", "displayName": "Application Administrator", "isBuiltIn": true }
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let assignments = list_role_assignments(&client).await.unwrap();

        // The service principal assignment must be filtered out; only user
        // principals are in scope for this tool.
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].user_id, "user-1");
        assert_eq!(assignments[0].user_principal_name, "alice@contoso.com");
        assert_eq!(assignments[0].role_name, "Global Administrator");
        assert_eq!(assignments[0].assignment_type, AssignmentType::Direct);
        assert!(assignments[0].is_permanent);
    }

    #[tokio::test]
    async fn falls_back_to_role_definition_id_when_display_name_is_missing() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/roleManagement/directory/roleAssignments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [{
                    "id": "assignment-1",
                    "roleDefinitionId": "role-1",
                    "principalId": "user-1",
                    "principal": {
                        "id": "user-1",
                        "userPrincipalName": "alice@contoso.com",
                        "@odata.type": "#microsoft.graph.user"
                    },
                    "roleDefinition": null
                }]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let assignments = list_role_assignments(&client).await.unwrap();

        assert_eq!(assignments[0].role_name, "role-1");
    }

    #[tokio::test]
    async fn assignment_without_a_principal_is_skipped() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/roleManagement/directory/roleAssignments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [{
                    "id": "assignment-1",
                    "roleDefinitionId": "role-1",
                    "principalId": null,
                    "principal": null,
                    "roleDefinition": null
                }]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let assignments = list_role_assignments(&client).await.unwrap();

        assert!(assignments.is_empty());
    }
}
