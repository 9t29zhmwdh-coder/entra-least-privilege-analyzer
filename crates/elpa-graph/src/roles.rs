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
