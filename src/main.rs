use create_rust_app::templates::write::{WebWrite, Write};
use log::{error, info};
use std::str::FromStr;
use structopt::StructOpt;
use anyhow::{Context, Result};

use create_rust_app::{add_dependencies, create_config_files, create_directories, create_project};
use create_rust_app::{
    cli::{Cli, ProjectType},
    state::State,
    templates::cli_template::CliTemplate,
    templates::web_template::WebTemplate,
};

fn main() -> Result<()> {
    env_logger::init();
    let mut state = State::Initialization;
    let args = Cli::from_args();

    loop {
        match state {
            State::Initialization => {
                info!("Creating project...");
                if let Err(e) = create_project(&args.get("name").unwrap(), "bin").context("Failed to create project") {
                    error!("{}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::DirectoryStructure;
                }
            }
            State::DirectoryStructure => {
                info!("Creating directories...");
                if let Err(e) = create_directories(&args.get("name").unwrap()).context("Failed to create directories") {
                    error!("{}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::ConfigurationFiles;
                }
            }
            State::ConfigurationFiles => {
                info!("Creating configuration files...");
                if let Err(e) = create_config_files(
                    &args.get("name").unwrap(),
                    args.get("license").unwrap_or(&String::from("MIT")),
                ).context("Failed to create configuration files") {
                    error!("{}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::Dependencies;
                }
            }
            State::Dependencies => {
                info!("Adding dependencies...");
                if let Err(e) = add_dependencies(
                    &args.get("name").unwrap(),
                    args.get("dependencies").as_deref().map(String::as_str),
                    args.get("dev_dependencies").as_deref().map(String::as_str),
                ).context("Failed to add dependencies") {
                    error!("{}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::CodeTemplates;
                }
            }
            State::CodeTemplates => {
                info!("Writing code templates...");
                let project_type = ProjectType::from_str(args.get("project_type").unwrap());
                match project_type {
                    Ok(project) => {
                        match project {
                            ProjectType::Cli => {
                                let template = CliTemplate;
                                template.write_main_rs(&args.get("name").unwrap()).context("Failed to write main.rs")?;
                                template.write_mod_rs(&args.get("name").unwrap()).context("Failed to write mod.rs")?;
                                template.write_utils_rs(&args.get("name").unwrap()).context("Failed to write utils.rs")?;
                                template.write_error_rs(&args.get("name").unwrap()).context("Failed to write error.rs")?;
                            }
                            ProjectType::Web => {
                                let template = WebTemplate;
                                template.write_main_rs(&args.get("name").unwrap()).context("Failed to write main.rs")?;
                                template.write_mod_rs(&args.get("name").unwrap()).context("Failed to write mod.rs")?;
                                template.write_utils_rs(&args.get("name").unwrap()).context("Failed to write utils.rs")?;
                                template.write_error_rs(&args.get("name").unwrap()).context("Failed to write error.rs")?;
                                template.write_server_rs(&args.get("name").unwrap()).context("Failed to write server.rs")?;
                                template.write_router_rs(&args.get("name").unwrap()).context("Failed to write router.rs")?;
                                template.write_handlers_rs(&args.get("name").unwrap()).context("Failed to write handlers.rs")?;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse project type: {}", e);
                        state = State::Error(e.to_string());
                    }
                    
                }
            }
            State::Customization => {
                state = State::Finalization;
            }
            State::Finalization => {
                info!("Project setup complete.");
                state = State::Done;
            }
            State::Done => {
                break;
            }
            State::Error(ref msg) => {
                error!("Error encountered: {}", msg);
                return Err(anyhow::anyhow!(msg.clone()));
            }
        }
        state = state.next();
    }
    Ok(())
}
