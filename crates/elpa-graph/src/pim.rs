use anyhow::Result;
use serde::Deserialize;

use crate::client::GraphClient;
use elpa_core::models::{AssignmentType, PimRoleSettings, RoleAssignment};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphEligibilitySchedule {
    id: String,
    principal_id: Option<String>,
    role_definition_id: Option<String>,
    role_definition: Option<PimRoleRef>,
    principal: Option<PimPrincipal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PimPrincipal {
    id: Option<String>,
    user_principal_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PimRoleRef {
    id: Option<String>,
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphRoleManagementPolicyAssignment {
    id: String,
    role_definition_id: Option<String>,
    role_definition: Option<PimRoleRef>,
    policy: Option<GraphRoleManagementPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphRoleManagementPolicy {
    id: String,
    rules: Option<Vec<serde_json::Value>>,
}

pub async fn list_pim_eligible_assignments(client: &GraphClient) -> Result<Vec<RoleAssignment>> {
    let raw: Vec<GraphEligibilitySchedule> = client
        .get_all_pages(
            "/roleManagement/directory/roleEligibilitySchedules?$expand=principal,roleDefinition",
        )
        .await?;

    let assignments = raw
        .into_iter()
        .filter_map(|a| {
            let principal = a.principal.as_ref()?;
            let user_id = principal.id.clone().unwrap_or_else(|| a.principal_id.clone().unwrap_or_default());
            let upn = principal
                .user_principal_name
                .clone()
                .unwrap_or_else(|| user_id.clone());
            let (role_id, role_name) = match &a.role_definition {
                Some(r) => (
                    r.id.clone().unwrap_or_else(|| a.role_definition_id.clone().unwrap_or_default()),
                    r.display_name.clone().unwrap_or_default(),
                ),
                None => (a.role_definition_id.clone().unwrap_or_default(), a.id.clone()),
            };

            Some(RoleAssignment {
                user_id,
                user_principal_name: upn,
                role_id,
                role_name,
                assignment_type: AssignmentType::PimEligible,
                is_permanent: false,
                expires_at: None,
            })
        })
        .collect();

    Ok(assignments)
}

pub async fn get_pim_role_settings(client: &GraphClient) -> Result<Vec<PimRoleSettings>> {
    let assignments: Vec<GraphRoleManagementPolicyAssignment> = client
        .get_all_pages(
            "/policies/roleManagementPolicyAssignments?$filter=scopeType eq 'DirectoryRole'&$expand=policy($expand=rules)",
        )
        .await
        .unwrap_or_default();

    let settings = assignments
        .into_iter()
        .filter_map(|a| {
            let role_name = a
                .role_definition
                .as_ref()
                .and_then(|r| r.display_name.clone())
                .unwrap_or_default();
            let role_id = a.role_definition_id.clone().unwrap_or_default();

            let rules = a.policy.as_ref().and_then(|p| p.rules.as_ref());
            let requires_mfa = extract_mfa_requirement(rules);
            let requires_justification = extract_justification_requirement(rules);
            let max_hours = extract_max_duration_hours(rules);

            Some(PimRoleSettings {
                role_id,
                role_name,
                requires_mfa,
                requires_approval: false,
                max_activation_duration_hours: max_hours,
                requires_justification,
            })
        })
        .collect();

    Ok(settings)
}

fn extract_mfa_requirement(rules: Option<&Vec<serde_json::Value>>) -> bool {
    rules.map_or(false, |r| {
        r.iter().any(|rule| {
            rule.get("@odata.type")
                .and_then(|t| t.as_str())
                .map_or(false, |t| t.contains("AuthenticationContext"))
                && rule
                    .get("isEnabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
        })
    })
}

fn extract_justification_requirement(rules: Option<&Vec<serde_json::Value>>) -> bool {
    rules.map_or(false, |r| {
        r.iter().any(|rule| {
            rule.get("@odata.type")
                .and_then(|t| t.as_str())
                .map_or(false, |t| t.contains("Justification"))
                && rule
                    .get("isRequired")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
        })
    })
}

fn extract_max_duration_hours(rules: Option<&Vec<serde_json::Value>>) -> u32 {
    rules
        .and_then(|r| {
            r.iter()
                .find(|rule| {
                    rule.get("@odata.type")
                        .and_then(|t| t.as_str())
                        .map_or(false, |t| t.contains("MaximumGrantPeriod"))
                })
                .and_then(|rule| rule.get("maximumDuration"))
                .and_then(|v| v.as_str())
                .and_then(parse_iso8601_duration_hours)
        })
        .unwrap_or(8)
}

fn parse_iso8601_duration_hours(s: &str) -> Option<u32> {
    // PT8H → 8, PT4H30M → 4
    if let Some(h_pos) = s.find('H') {
        let start = s.find('T').map(|p| p + 1).unwrap_or(0);
        s[start..h_pos].parse::<u32>().ok()
    } else {
        None
    }
}
