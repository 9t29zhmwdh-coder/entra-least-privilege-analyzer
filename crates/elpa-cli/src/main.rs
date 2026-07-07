use anyhow::Result;
use clap::{Parser, Subcommand};
use elpa_core::{analyzer, report};
use elpa_graph::{client::GraphClient, pim, roles, users};
use tabled::{Table, Tabled};
use tracing::info;

#[derive(Parser)]
#[command(
    name = "elpa",
    version = "0.1.0",
    author = "RayStudio",
    about = "Entra Least-Privilege Analyzer: read-only Entra ID privilege analysis"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a full tenant privilege analysis
    Analyze {
        /// Privilege score threshold for over-privileged detection
        #[arg(long, default_value_t = 100)]
        threshold: u32,
    },
    /// Analyze PIM configuration only
    Pim,
    /// Export full analysis as JSON or Markdown
    Export {
        /// Output format: json or md
        #[arg(long, short, default_value = "json")]
        format: String,
        /// Output file path (defaults to stdout)
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Run the analysis against a built-in synthetic tenant, no Entra ID credentials needed
    Demo,
}

#[derive(Tabled)]
struct ScoreRow {
    #[tabled(rename = "Account")]
    account: String,
    #[tabled(rename = "Score")]
    score: u32,
    #[tabled(rename = "Flags")]
    flags: String,
}

#[derive(Tabled)]
struct GapRow {
    #[tabled(rename = "Severity")]
    severity: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Affected")]
    affected: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("elpa=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    if matches!(cli.command, Command::Demo) {
        run_demo();
        return Ok(());
    }

    let tenant_id = std::env::var("ENTRA_TENANT_ID").unwrap_or_else(|_| "unknown".to_string());
    let client = GraphClient::from_env()?;

    match cli.command {
        Command::Analyze { threshold } => {
            info!("Starting full tenant analysis for {}", tenant_id);
            let (users, assignments, pim_settings) = fetch_all(&client).await?;
            let result = analyzer::build_analysis_result(
                tenant_id,
                &users,
                &assignments,
                &pim_settings,
            );

            println!("\n=== Entra Least-Privilege Analyzer ===\n");
            println!(
                "Users: {}  Assignments: {}  Findings: {}\n",
                result.user_count,
                result.assignment_count,
                result.gaps.len()
            );

            if !result.gaps.is_empty() {
                let gap_rows: Vec<GapRow> = result
                    .gaps
                    .iter()
                    .map(|g| GapRow {
                        severity: g.severity.to_string(),
                        title: truncate(&g.title, 60),
                        affected: g.affected_principals.len(),
                    })
                    .collect();
                println!("Findings:\n{}\n", Table::new(gap_rows));
            }

            let over = analyzer::detect_overprivileged(&result.scores, threshold);
            if !over.is_empty() {
                let rows: Vec<ScoreRow> = over
                    .iter()
                    .take(10)
                    .map(|s| ScoreRow {
                        account: s.user_principal_name.clone(),
                        score: s.score,
                        flags: s
                            .flags
                            .iter()
                            .map(|f| format!("{:?}", f))
                            .collect::<Vec<_>>()
                            .join(", "),
                    })
                    .collect();
                println!("Over-privileged accounts (score >= {}):\n{}", threshold, Table::new(rows));
            } else {
                println!("No accounts exceeded the privilege threshold of {}.", threshold);
            }

            println!(
                "\nSummary: {} Critical, {} High, {} Medium, {} Low",
                result.summary.critical_count,
                result.summary.high_count,
                result.summary.medium_count,
                result.summary.low_count
            );
        }

        Command::Pim => {
            info!("Running PIM-only analysis for {}", tenant_id);
            let (_, assignments, pim_settings) = fetch_all(&client).await?;
            let gaps = analyzer::analyze_pim(&assignments, &pim_settings);

            if gaps.is_empty() {
                println!("No PIM configuration gaps detected.");
            } else {
                let rows: Vec<GapRow> = gaps
                    .iter()
                    .map(|g| GapRow {
                        severity: g.severity.to_string(),
                        title: truncate(&g.title, 60),
                        affected: g.affected_principals.len(),
                    })
                    .collect();
                println!("PIM Findings:\n{}", Table::new(rows));
            }
        }

        Command::Export { format, output } => {
            info!("Exporting analysis for {}", tenant_id);
            let (users, assignments, pim_settings) = fetch_all(&client).await?;
            let result = analyzer::build_analysis_result(
                tenant_id,
                &users,
                &assignments,
                &pim_settings,
            );

            let content = match format.as_str() {
                "md" => report::to_markdown(&result),
                _ => report::to_json(&result)?,
            };

            match output {
                Some(path) => std::fs::write(&path, &content)?,
                None => print!("{}", content),
            }
        }

        Command::Demo => unreachable!("handled before GraphClient is created"),
    }

    Ok(())
}

fn run_demo() {
    use elpa_core::models::{AssignmentType, PimRoleSettings, RoleAssignment, User};

    let users = vec![
        User { id: "u1".into(), display_name: "Admin Contoso".into(), user_principal_name: "admin@contoso.com".into(), account_enabled: true },
        User { id: "u2".into(), display_name: "Ops Contoso".into(), user_principal_name: "ops@contoso.com".into(), account_enabled: true },
    ];
    let assignments = vec![
        RoleAssignment { user_id: "u1".into(), user_principal_name: "admin@contoso.com".into(), role_id: "role-global-admin".into(), role_name: "Global Administrator".into(), assignment_type: AssignmentType::Direct, is_permanent: true, expires_at: None },
        RoleAssignment { user_id: "u2".into(), user_principal_name: "ops@contoso.com".into(), role_id: "role-user-admin".into(), role_name: "User Administrator".into(), assignment_type: AssignmentType::Direct, is_permanent: false, expires_at: None },
        RoleAssignment { user_id: "u2".into(), user_principal_name: "ops@contoso.com".into(), role_id: "role-exchange-admin".into(), role_name: "Exchange Administrator".into(), assignment_type: AssignmentType::Direct, is_permanent: false, expires_at: None },
    ];
    let pim_settings = vec![PimRoleSettings {
        role_id: "role-global-admin".into(),
        role_name: "Global Administrator".into(),
        requires_mfa: false,
        requires_approval: false,
        max_activation_duration_hours: 8,
        requires_justification: false,
    }];

    let result = analyzer::build_analysis_result(
        "contoso-demo-tenant".to_string(),
        &users,
        &assignments,
        &pim_settings,
    );

    println!("\n=== Entra Least-Privilege Analyzer (demo, synthetic data) ===\n");
    println!(
        "Users: {}  Assignments: {}  Findings: {}\n",
        result.user_count,
        result.assignment_count,
        result.gaps.len()
    );

    if !result.gaps.is_empty() {
        let gap_rows: Vec<GapRow> = result
            .gaps
            .iter()
            .map(|g| GapRow {
                severity: g.severity.to_string(),
                title: truncate(&g.title, 60),
                affected: g.affected_principals.len(),
            })
            .collect();
        println!("Findings:\n{}\n", Table::new(gap_rows));
    }

    println!(
        "Summary: {} Critical, {} High, {} Medium, {} Low",
        result.summary.critical_count,
        result.summary.high_count,
        result.summary.medium_count,
        result.summary.low_count
    );
}

async fn fetch_all(
    client: &GraphClient,
) -> Result<(
    Vec<elpa_core::models::User>,
    Vec<elpa_core::models::RoleAssignment>,
    Vec<elpa_core::models::PimRoleSettings>,
)> {
    let users = users::list_users(client).await?;
    let mut assignments = roles::list_role_assignments(client).await?;
    let pim_eligible = pim::list_pim_eligible_assignments(client).await?;
    assignments.extend(pim_eligible);
    let pim_settings = pim::get_pim_role_settings(client).await?;
    Ok((users, assignments, pim_settings))
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
