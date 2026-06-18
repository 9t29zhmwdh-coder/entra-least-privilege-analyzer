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
