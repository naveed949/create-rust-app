pub mod cli;
pub mod state;
pub mod templates;
pub mod writer;

use std::fs;
use std::process::Command;
use log::info;
use anyhow::{Context, Result};

pub fn create_project(name: &str, project_type: &str) -> Result<()> {
    info!("Creating project: {} of type: {}", name, project_type);
    Command::new("cargo")
        .arg("new")
        .arg(name)
        .arg(if project_type == "lib" { "--lib" } else { "--bin" })
        .status()
        .context("Failed to create new cargo project")?;
    Ok(())
}

pub fn create_directories(name: &str) -> Result<()> {
    let dirs = ["src", "tests", "examples", "benches"];
    for dir in &dirs {
        let path = format!("{}/{}", name, dir);
        info!("Creating directory: {}", path);
        fs::create_dir_all(&path).context(format!("Failed to create directory: {}", path))?;
    }
    Ok(())
}

pub fn create_config_files(name: &str, license: &str) -> Result<()> {
    info!("Creating configuration files for project: {}", name);
    fs::write(format!("{}/README.md", name), "# Project\n").context("Failed to create README.md")?;
    fs::write(format!("{}/LICENSE", name), license).context("Failed to create LICENSE file")?;
    fs::write(format!("{}/.gitignore", name), "target/\n").context("Failed to create .gitignore file")?;
    Ok(())
}

pub fn add_dependencies(name: &str, dependencies: Option<&str>, dev_dependencies: Option<&str>) -> Result<()> {
    info!("Adding dependencies to project: {}", name);
    let mut cargo_toml = fs::read_to_string(format!("{}/Cargo.toml", name)).context("Failed to read Cargo.toml")?;
    let common_deps = r#"
    log = "0.4"
    clap = "2.33"
    thiserror = "1.0"
    anyhow = "1.0"
    "#;

    // Add common dependencies
    cargo_toml.push_str(&format!("{}", common_deps));
    if let Some(deps) = dependencies {
        cargo_toml.push_str(&format!("{}", deps.replace(",", "\n")));
    }
    if let Some(dev_deps) = dev_dependencies {
        cargo_toml.push_str(&format!("\n[dev-dependencies]\n{}", dev_deps.replace(",", "\n")));
    }
    fs::write(format!("{}/Cargo.toml", name), cargo_toml).context("Failed to write to Cargo.toml")?;
    Ok(())
}

