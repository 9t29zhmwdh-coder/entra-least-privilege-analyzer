//! Example: export analysis results to a Markdown file.
//!
//! cargo run --example export_report -- --output report.md

use anyhow::Result;
use elpa_core::{analyzer, report};
use elpa_graph::{client::GraphClient, pim, roles, users};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let output_path = std::env::args().nth(2).unwrap_or_else(|| "report.md".to_string());
    let tenant_id = std::env::var("ENTRA_TENANT_ID").unwrap_or_else(|_| "unknown".to_string());
    let client = GraphClient::from_env()?;

    let all_users = users::list_users(&client).await?;
    let mut assignments = roles::list_role_assignments(&client).await?;
    assignments.extend(pim::list_pim_eligible_assignments(&client).await?);
    let pim_settings = pim::get_pim_role_settings(&client).await?;

    let result = analyzer::build_analysis_result(
        tenant_id,
        &all_users,
        &assignments,
        &pim_settings,
    );

    let md = report::to_markdown(&result);
    std::fs::write(&output_path, md)?;
    println!("Report written to {}", output_path);

    Ok(())
}
