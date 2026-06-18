use std::collections::HashMap;

use crate::models::{
    AnalysisResult, AnalysisSummary, AssignmentType, GapType, PimRoleSettings, PrivilegeFlag,
    PrivilegeScore, RoleAssignment, SecurityGap, Severity, User,
};
use chrono::Utc;

const SCORE_GLOBAL_ADMIN: u32 = 100;
const SCORE_PRIVILEGED_ROLE_ADMIN: u32 = 80;
const SCORE_SECURITY_ADMIN: u32 = 60;
const SCORE_EXCHANGE_ADMIN: u32 = 50;
const SCORE_USER_ADMIN: u32 = 40;
const SCORE_GENERIC_HIGH: u32 = 30;
const SCORE_PERMANENT_BONUS: u32 = 20;

const HIGH_PRIVILEGE_ROLES: &[&str] = &[
    "Global Administrator",
    "Privileged Role Administrator",
    "Security Administrator",
    "Exchange Administrator",
    "SharePoint Administrator",
    "Conditional Access Administrator",
    "Authentication Administrator",
    "Hybrid Identity Administrator",
];

pub fn compute_privilege_scores(
    users: &[User],
    assignments: &[RoleAssignment],
) -> Vec<PrivilegeScore> {
    let mut scores: HashMap<String, PrivilegeScore> = users
        .iter()
        .map(|u| {
            (
                u.id.clone(),
                PrivilegeScore {
                    user_id: u.id.clone(),
                    user_principal_name: u.user_principal_name.clone(),
                    score: 0,
                    flags: vec![],
                },
            )
        })
        .collect();

    for assignment in assignments {
        let entry = scores.entry(assignment.user_id.clone()).or_insert(PrivilegeScore {
            user_id: assignment.user_id.clone(),
            user_principal_name: assignment.user_principal_name.clone(),
            score: 0,
            flags: vec![],
        });

        let role_score = match assignment.role_name.as_str() {
            "Global Administrator" => {
                entry.flags.push(PrivilegeFlag::GlobalAdmin);
                SCORE_GLOBAL_ADMIN
            }
            "Privileged Role Administrator" => {
                entry.flags.push(PrivilegeFlag::PrivilegedRoleAdmin);
                SCORE_PRIVILEGED_ROLE_ADMIN
            }
            "Security Administrator" => {
                entry.flags.push(PrivilegeFlag::SecurityAdmin);
                SCORE_SECURITY_ADMIN
            }
            "Exchange Administrator" => {
                entry.flags.push(PrivilegeFlag::ExchangeAdmin);
                SCORE_EXCHANGE_ADMIN
            }
            "User Administrator" => SCORE_USER_ADMIN,
            _ => SCORE_GENERIC_HIGH,
        };

        entry.score += role_score;

        if assignment.is_permanent
            && HIGH_PRIVILEGE_ROLES.contains(&assignment.role_name.as_str())
        {
            entry.score += SCORE_PERMANENT_BONUS;
            if !entry.flags.contains(&PrivilegeFlag::PermanentHighPrivilege) {
                entry.flags.push(PrivilegeFlag::PermanentHighPrivilege);
            }
        }
    }

    let mut result: Vec<PrivilegeScore> = scores.into_values().collect();
    result.sort_by_key(|b| std::cmp::Reverse(b.score));
    result
}

pub fn detect_overprivileged(scores: &[PrivilegeScore], threshold: u32) -> Vec<&PrivilegeScore> {
    scores.iter().filter(|s| s.score >= threshold).collect()
}

pub fn find_role_overlap(
    assignments: &[RoleAssignment],
) -> Vec<(String, Vec<String>)> {
    let mut user_roles: HashMap<String, Vec<String>> = HashMap::new();
    for a in assignments {
        user_roles
            .entry(a.user_principal_name.clone())
            .or_default()
            .push(a.role_name.clone());
    }

    user_roles
        .into_iter()
        .filter(|(_, roles)| roles.len() >= 3)
        .collect()
}

pub fn analyze_pim(
    assignments: &[RoleAssignment],
    settings: &[PimRoleSettings],
) -> Vec<SecurityGap> {
    let mut gaps = vec![];

    let permanent_high_priv: Vec<&RoleAssignment> = assignments
        .iter()
        .filter(|a| {
            a.is_permanent
                && HIGH_PRIVILEGE_ROLES.contains(&a.role_name.as_str())
                && a.assignment_type == AssignmentType::Direct
        })
        .collect();

    if !permanent_high_priv.is_empty() {
        gaps.push(SecurityGap {
            gap_type: GapType::PermanentHighPrivilege,
            severity: Severity::Critical,
            title: "Permanent high-privilege role assignments without PIM".to_string(),
            description: format!(
                "{} account(s) hold high-privilege roles as permanent direct assignments. Microsoft recommends using Privileged Identity Management (PIM) for just-in-time access.",
                permanent_high_priv.len()
            ),
            affected_principals: permanent_high_priv
                .iter()
                .map(|a| a.user_principal_name.clone())
                .collect(),
            remediation: "Migrate permanent role assignments to PIM eligible assignments. Require MFA and justification for activation.".to_string(),
        });
    }

    for setting in settings {
        if HIGH_PRIVILEGE_ROLES.contains(&setting.role_name.as_str()) {
            if !setting.requires_mfa {
                gaps.push(SecurityGap {
                    gap_type: GapType::PimWeakSettings,
                    severity: Severity::High,
                    title: format!("PIM activation for '{}' does not require MFA", setting.role_name),
                    description: "MFA is not enforced during PIM role activation, reducing the effectiveness of just-in-time access controls.".to_string(),
                    affected_principals: vec![setting.role_name.clone()],
                    remediation: "Enable MFA requirement in PIM role settings for this role.".to_string(),
                });
            }

            if setting.max_activation_duration_hours > 8 {
                gaps.push(SecurityGap {
                    gap_type: GapType::PimWeakSettings,
                    severity: Severity::Medium,
                    title: format!(
                        "PIM activation duration for '{}' exceeds 8 hours ({}h configured)",
                        setting.role_name, setting.max_activation_duration_hours
                    ),
                    description: "Long activation windows increase the exposure time for privileged operations.".to_string(),
                    affected_principals: vec![setting.role_name.clone()],
                    remediation: "Reduce maximum activation duration to 4-8 hours based on operational requirements.".to_string(),
                });
            }

            if !setting.requires_justification {
                gaps.push(SecurityGap {
                    gap_type: GapType::PimWeakSettings,
                    severity: Severity::Low,
                    title: format!(
                        "PIM activation for '{}' does not require justification",
                        setting.role_name
                    ),
                    description: "Activation justification provides an audit trail and accountability for privileged access.".to_string(),
                    affected_principals: vec![setting.role_name.clone()],
                    remediation: "Enable justification requirement in PIM role settings.".to_string(),
                });
            }
        }
    }

    gaps
}

pub fn build_analysis_result(
    tenant_id: String,
    users: &[User],
    assignments: &[RoleAssignment],
    pim_settings: &[PimRoleSettings],
) -> AnalysisResult {
    let scores = compute_privilege_scores(users, assignments);
    let mut gaps = analyze_pim(assignments, pim_settings);

    let overlaps = find_role_overlap(assignments);
    for (upn, roles) in &overlaps {
        gaps.push(SecurityGap {
            gap_type: GapType::RoleOverlap,
            severity: Severity::Medium,
            title: format!("Role overlap detected for {}", upn),
            description: format!(
                "Account holds {} roles simultaneously: {}. Review whether all assignments are required.",
                roles.len(),
                roles.join(", ")
            ),
            affected_principals: vec![upn.clone()],
            remediation: "Review and remove redundant role assignments following the principle of least privilege.".to_string(),
        });
    }

    let overprivileged = detect_overprivileged(&scores, 120);
    for score in &overprivileged {
        if !gaps.iter().any(|g| {
            g.affected_principals.contains(&score.user_principal_name)
                && matches!(g.gap_type, GapType::OverprivilegedAccount)
        }) {
            gaps.push(SecurityGap {
                gap_type: GapType::OverprivilegedAccount,
                severity: Severity::High,
                title: format!("Over-privileged account: {}", score.user_principal_name),
                description: format!(
                    "Privilege score of {} exceeds recommended threshold. Account holds multiple high-impact roles.",
                    score.score
                ),
                affected_principals: vec![score.user_principal_name.clone()],
                remediation: "Review all role assignments for this account and remove roles that are not required for day-to-day operations.".to_string(),
            });
        }
    }

    let summary = AnalysisSummary::from_gaps(&gaps, &scores);

    AnalysisResult {
        tenant_id,
        analyzed_at: Utc::now(),
        user_count: users.len(),
        assignment_count: assignments.len(),
        scores,
        gaps,
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;

    fn make_user(id: &str, upn: &str) -> User {
        User {
            id: id.to_string(),
            display_name: upn.to_string(),
            user_principal_name: upn.to_string(),
            account_enabled: true,
        }
    }

    fn make_assignment(user_id: &str, upn: &str, role: &str, permanent: bool) -> RoleAssignment {
        RoleAssignment {
            user_id: user_id.to_string(),
            user_principal_name: upn.to_string(),
            role_id: format!("role-{}", role),
            role_name: role.to_string(),
            assignment_type: AssignmentType::Direct,
            is_permanent: permanent,
            expires_at: None,
        }
    }

    #[test]
    fn test_global_admin_scores_highest() {
        let users = vec![make_user("u1", "admin@contoso.com")];
        let assignments = vec![make_assignment("u1", "admin@contoso.com", "Global Administrator", true)];
        let scores = compute_privilege_scores(&users, &assignments);
        assert!(!scores.is_empty());
        assert!(scores[0].score >= SCORE_GLOBAL_ADMIN + SCORE_PERMANENT_BONUS);
        assert!(scores[0].flags.contains(&PrivilegeFlag::GlobalAdmin));
        assert!(scores[0].flags.contains(&PrivilegeFlag::PermanentHighPrivilege));
    }

    #[test]
    fn test_detect_overprivileged_threshold() {
        let scores = vec![
            PrivilegeScore { user_id: "u1".to_string(), user_principal_name: "a@b.com".to_string(), score: 150, flags: vec![] },
            PrivilegeScore { user_id: "u2".to_string(), user_principal_name: "c@b.com".to_string(), score: 50, flags: vec![] },
        ];
        let over = detect_overprivileged(&scores, 100);
        assert_eq!(over.len(), 1);
        assert_eq!(over[0].user_id, "u1");
    }

    #[test]
    fn test_pim_gap_no_mfa() {
        let assignments: Vec<RoleAssignment> = vec![];
        let settings = vec![PimRoleSettings {
            role_id: "r1".to_string(),
            role_name: "Global Administrator".to_string(),
            requires_mfa: false,
            requires_approval: true,
            max_activation_duration_hours: 4,
            requires_justification: true,
        }];
        let gaps = analyze_pim(&assignments, &settings);
        assert!(gaps.iter().any(|g| matches!(g.gap_type, GapType::PimWeakSettings)
            && g.severity == Severity::High));
    }
}
