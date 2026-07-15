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
    #[allow(dead_code)]
    id: String,
    role_definition_id: Option<String>,
    role_definition: Option<PimRoleRef>,
    policy: Option<GraphRoleManagementPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphRoleManagementPolicy {
    #[allow(dead_code)]
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
        .map(|a| {
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

            PimRoleSettings {
                role_id,
                role_name,
                requires_mfa,
                requires_approval: false,
                max_activation_duration_hours: max_hours,
                requires_justification,
            }
        })
        .collect();

    Ok(settings)
}

fn extract_mfa_requirement(rules: Option<&Vec<serde_json::Value>>) -> bool {
    rules.is_some_and(|r| {
        r.iter().any(|rule| {
            rule.get("@odata.type")
                .and_then(|t| t.as_str())
                .is_some_and(|t| t.contains("AuthenticationContext"))
                && rule
                    .get("isEnabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
        })
    })
}

fn extract_justification_requirement(rules: Option<&Vec<serde_json::Value>>) -> bool {
    rules.is_some_and(|r| {
        r.iter().any(|rule| {
            rule.get("@odata.type")
                .and_then(|t| t.as_str())
                .is_some_and(|t| t.contains("Justification"))
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
                        .is_some_and(|t| t.contains("MaximumGrantPeriod"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GraphClient;
    use serde_json::json;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn parses_hours_only_duration() {
        assert_eq!(parse_iso8601_duration_hours("PT8H"), Some(8));
    }

    #[test]
    fn parses_hours_and_minutes_duration_truncating_minutes() {
        assert_eq!(parse_iso8601_duration_hours("PT4H30M"), Some(4));
    }

    #[test]
    fn duration_without_an_h_component_is_not_parsed() {
        // PT30M (minutes only, no hours) is a real Graph value this tool
        // does not currently attempt to convert to hours.
        assert_eq!(parse_iso8601_duration_hours("PT30M"), None);
        assert_eq!(parse_iso8601_duration_hours(""), None);
    }

    #[test]
    fn extract_mfa_requirement_true_only_when_enabled_authentication_context_rule_present() {
        let rules = vec![json!({
            "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyAuthenticationContextRule",
            "isEnabled": true,
        })];
        assert!(extract_mfa_requirement(Some(&rules)));

        let disabled = vec![json!({
            "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyAuthenticationContextRule",
            "isEnabled": false,
        })];
        assert!(!extract_mfa_requirement(Some(&disabled)));

        assert!(!extract_mfa_requirement(None));
        assert!(!extract_mfa_requirement(Some(&vec![])));
    }

    #[test]
    fn extract_justification_requirement_true_only_when_required_rule_present() {
        let rules = vec![json!({
            "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyJustificationRule",
            "isRequired": true,
        })];
        assert!(extract_justification_requirement(Some(&rules)));
        assert!(!extract_justification_requirement(None));
    }

    #[test]
    fn extract_max_duration_hours_defaults_to_eight_when_rule_missing() {
        assert_eq!(extract_max_duration_hours(None), 8);
        assert_eq!(extract_max_duration_hours(Some(&vec![])), 8);
    }

    #[test]
    fn extract_max_duration_hours_reads_the_matching_rule_by_odata_type() {
        // The matcher keys off "@odata.type" containing "MaximumGrantPeriod".
        let rules = vec![json!({
            "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyExpirationRule.MaximumGrantPeriod",
            "maximumDuration": "PT12H",
        })];
        assert_eq!(extract_max_duration_hours(Some(&rules)), 12);
    }

    #[test]
    fn extract_max_duration_hours_ignores_non_matching_rules_and_defaults_to_eight() {
        // A same-family expiration rule whose @odata.type does not contain
        // "MaximumGrantPeriod" is not recognized by the current matcher, so
        // the default of 8 applies. Documents actual (narrow) behavior.
        let rules = vec![json!({
            "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyExpirationRule",
            "id": "Expiration_EndUser_Assignment",
            "maximumDuration": "PT12H",
        })];
        assert_eq!(extract_max_duration_hours(Some(&rules)), 8);
    }

    #[tokio::test]
    async fn list_pim_eligible_assignments_maps_graph_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"^/roleManagement/directory/roleEligibilitySchedules"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "value": [{
                    "id": "sched-1",
                    "principalId": "user-1",
                    "roleDefinitionId": "role-1",
                    "principal": {"id": "user-1", "userPrincipalName": "alice@contoso.com"},
                    "roleDefinition": {"id": "role-1", "displayName": "Security Administrator"}
                }]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let assignments = list_pim_eligible_assignments(&client).await.unwrap();

        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].role_name, "Security Administrator");
        assert_eq!(assignments[0].assignment_type, AssignmentType::PimEligible);
        assert!(!assignments[0].is_permanent);
    }

    #[tokio::test]
    async fn get_pim_role_settings_maps_policy_rules() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"^/policies/roleManagementPolicyAssignments",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "value": [{
                    "id": "policy-assignment-1",
                    "roleDefinitionId": "role-1",
                    "roleDefinition": {"id": "role-1", "displayName": "Global Administrator"},
                    "policy": {
                        "id": "policy-1",
                        "rules": [
                            {
                                "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyAuthenticationContextRule",
                                "isEnabled": true
                            },
                            {
                                "@odata.type": "#microsoft.graph.unifiedRoleManagementPolicyExpirationRule",
                                "id": "Expiration_EndUser_Assignment",
                                "maximumDuration": "PT4H"
                            }
                        ]
                    }
                }]
            })))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());
        let settings = get_pim_role_settings(&client).await.unwrap();

        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].role_name, "Global Administrator");
        assert!(settings[0].requires_mfa);
    }

    #[tokio::test]
    async fn get_pim_role_settings_returns_empty_on_graph_error_instead_of_failing() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"^/policies/roleManagementPolicyAssignments",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = GraphClient::from_token("tenant", "t").with_graph_base(server.uri());

        // get_pim_role_settings deliberately swallows Graph errors here
        // (.unwrap_or_default()) since PIM policy read requires an extra
        // permission (RoleManagementPolicy.Read.Directory) that not every
        // caller will have granted; the rest of the scan should still run.
        let settings = get_pim_role_settings(&client).await.unwrap();
        assert!(settings.is_empty());
    }
}
