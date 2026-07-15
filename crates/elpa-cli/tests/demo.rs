use assert_cmd::Command;

#[test]
fn demo_subcommand_runs_fully_offline_and_prints_synthetic_findings() {
    Command::cargo_bin("elpa")
        .unwrap()
        .arg("demo")
        .assert()
        .success()
        .stdout(predicates::str::contains("demo, synthetic data"))
        .stdout(predicates::str::contains("Users:"))
        .stdout(predicates::str::contains("Summary:"));
}

#[test]
fn version_flag_reports_the_actual_crate_version_not_a_stale_literal() {
    Command::cargo_bin("elpa")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn analyze_without_credentials_fails_cleanly_instead_of_panicking() {
    Command::cargo_bin("elpa")
        .unwrap()
        .arg("analyze")
        .env_remove("ENTRA_TENANT_ID")
        .env_remove("ENTRA_CLIENT_ID")
        .env_remove("ENTRA_CLIENT_SECRET")
        .env_remove("ENTRA_ACCESS_TOKEN")
        .assert()
        .failure()
        .stderr(predicates::str::contains("ENTRA_TENANT_ID"));
}
