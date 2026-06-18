//! Example: run a full tenant analysis and print findings to stdout.
//!
//! Set ENTRA_TENANT_ID, ENTRA_CLIENT_ID, ENTRA_CLIENT_SECRET before running.
//! cargo run --example analyze_tenant

use anyhow::Result;
use elpa_core::analyzer;
use elpa_graph::{client::GraphClient, pim, roles, users};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let tenant_id = std::env::var("ENTRA_TENANT_ID").unwrap_or_else(|_| "unknown".to_string());
    let client = GraphClient::from_env()?;

    println!("Fetching data from tenant {}...", tenant_id);

    let all_users = users::list_users(&client).await?;
    let mut assignments = roles::list_role_assignments(&client).await?;
    let eligible = pim::list_pim_eligible_assignments(&client).await?;
    assignments.extend(eligible);
    let pim_settings = pim::get_pim_role_settings(&client).await?;

    println!(
        "Found {} users and {} assignments.",
        all_users.len(),
        assignments.len()
    );

    let result = analyzer::build_analysis_result(
        tenant_id,
        &all_users,
        &assignments,
        &pim_settings,
    );

    println!(
        "\nFindings: {} Critical, {} High, {} Medium, {} Low",
        result.summary.critical_count,
        result.summary.high_count,
        result.summary.medium_count,
        result.summary.low_count,
    );

    for gap in &result.gaps {
        println!("[{}] {}", gap.severity, gap.title);
    }

    Ok(())
}
