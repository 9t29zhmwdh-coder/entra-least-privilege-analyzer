use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub user_principal_name: String,
    pub account_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentType {
    Direct,
    GroupBased,
    PimEligible,
    PimActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub user_id: String,
    pub user_principal_name: String,
    pub role_id: String,
    pub role_name: String,
    pub assignment_type: AssignmentType,
    pub is_permanent: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrivilegeFlag {
    GlobalAdmin,
    PrivilegedRoleAdmin,
    SecurityAdmin,
    ExchangeAdmin,
    RoleOverlap,
    NoPimProtection,
    PermanentHighPrivilege,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeScore {
    pub user_id: String,
    pub user_principal_name: String,
    pub score: u32,
    pub flags: Vec<PrivilegeFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
            Severity::Informational => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapType {
    OverprivilegedAccount,
    RoleOverlap,
    PimNotConfigured,
    PermanentHighPrivilege,
    PimWeakSettings,
    ConditionalAccessGap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGap {
    pub gap_type: GapType,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub affected_principals: Vec<String>,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PimRoleSettings {
    pub role_id: String,
    pub role_name: String,
    pub requires_mfa: bool,
    pub requires_approval: bool,
    pub max_activation_duration_hours: u32,
    pub requires_justification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub tenant_id: String,
    pub analyzed_at: DateTime<Utc>,
    pub user_count: usize,
    pub assignment_count: usize,
    pub scores: Vec<PrivilegeScore>,
    pub gaps: Vec<SecurityGap>,
    pub summary: AnalysisSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub overprivileged_accounts: usize,
    pub permanent_high_privilege_accounts: usize,
    pub roles_without_pim: usize,
}

impl AnalysisSummary {
    pub fn from_gaps(gaps: &[SecurityGap], scores: &[PrivilegeScore]) -> Self {
        Self {
            critical_count: gaps.iter().filter(|g| g.severity == Severity::Critical).count(),
            high_count: gaps.iter().filter(|g| g.severity == Severity::High).count(),
            medium_count: gaps.iter().filter(|g| g.severity == Severity::Medium).count(),
            low_count: gaps.iter().filter(|g| g.severity == Severity::Low).count(),
            overprivileged_accounts: scores.iter().filter(|s| s.score >= 100).count(),
            permanent_high_privilege_accounts: scores
                .iter()
                .filter(|s| s.flags.contains(&PrivilegeFlag::PermanentHighPrivilege))
                .count(),
            roles_without_pim: gaps
                .iter()
                .filter(|g| matches!(g.gap_type, GapType::PimNotConfigured))
                .count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gap(gap_type: GapType, severity: Severity) -> SecurityGap {
        SecurityGap {
            gap_type,
            severity,
            title: "t".to_string(),
            description: "d".to_string(),
            affected_principals: vec![],
            remediation: "r".to_string(),
        }
    }

    fn score(value: u32, flags: Vec<PrivilegeFlag>) -> PrivilegeScore {
        PrivilegeScore {
            user_id: "u".to_string(),
            user_principal_name: "u@contoso.com".to_string(),
            score: value,
            flags,
        }
    }

    #[test]
    fn from_gaps_counts_severities_independently() {
        let gaps = vec![
            gap(GapType::OverprivilegedAccount, Severity::Critical),
            gap(GapType::RoleOverlap, Severity::High),
            gap(GapType::RoleOverlap, Severity::High),
            gap(GapType::PimNotConfigured, Severity::Medium),
            gap(GapType::PimWeakSettings, Severity::Low),
        ];

        let summary = AnalysisSummary::from_gaps(&gaps, &[]);

        assert_eq!(summary.critical_count, 1);
        assert_eq!(summary.high_count, 2);
        assert_eq!(summary.medium_count, 1);
        assert_eq!(summary.low_count, 1);
    }

    #[test]
    fn from_gaps_counts_overprivileged_accounts_at_the_100_threshold() {
        let scores = vec![
            score(100, vec![]),
            score(99, vec![]),
            score(150, vec![]),
        ];

        let summary = AnalysisSummary::from_gaps(&[], &scores);

        assert_eq!(summary.overprivileged_accounts, 2);
    }

    #[test]
    fn from_gaps_counts_permanent_high_privilege_by_flag() {
        let scores = vec![
            score(10, vec![PrivilegeFlag::PermanentHighPrivilege]),
            score(10, vec![PrivilegeFlag::GlobalAdmin]),
        ];

        let summary = AnalysisSummary::from_gaps(&[], &scores);

        assert_eq!(summary.permanent_high_privilege_accounts, 1);
    }

    #[test]
    fn from_gaps_counts_roles_without_pim_by_gap_type_only() {
        let gaps = vec![
            gap(GapType::PimNotConfigured, Severity::High),
            gap(GapType::PimWeakSettings, Severity::High),
        ];

        let summary = AnalysisSummary::from_gaps(&gaps, &[]);

        assert_eq!(summary.roles_without_pim, 1);
    }

    #[test]
    fn from_gaps_on_empty_input_is_all_zero() {
        let summary = AnalysisSummary::from_gaps(&[], &[]);

        assert_eq!(summary.critical_count, 0);
        assert_eq!(summary.overprivileged_accounts, 0);
        assert_eq!(summary.roles_without_pim, 0);
    }
}
