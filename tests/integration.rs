use create_rust_app::{create_project, create_directories, create_config_files, add_dependencies};

#[test]
fn test_full_workflow() {
    let project_name = "integration_test_project";

    // Test project creation
    let result = create_project(project_name, "bin");
    assert!(result.is_ok());

    // Test directory creation
    let result = create_directories(project_name);
    assert!(result.is_ok());

    // Test configuration file creation
    let result = create_config_files(project_name, "MIT");
    assert!(result.is_ok());

    // Test adding dependencies
    let result = add_dependencies(project_name, None, None);
    assert!(result.is_ok());
}
