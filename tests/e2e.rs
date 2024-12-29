use std::process::Command;

#[test]
fn test_create_rust_app_cli() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--name")
        .arg("e2e_test_project")
        .arg("--project-type")
        .arg("cli")
        .arg("--license")
        .arg("MIT")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Project setup complete."));
}
