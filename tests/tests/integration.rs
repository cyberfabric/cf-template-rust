use cf_template_tests::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_template_structure() {
    let template_dir = template_dir();
    validation::validate_init_template(&template_dir).expect("Init template validation failed");
}

#[test]
fn test_modules_template_structure() {
    let template_dir = template_dir();
    validation::validate_modules_template(&template_dir).expect("Modules template validation failed");
}

#[test]
fn test_placeholders() {
    let template_dir = template_dir();
    validation::validate_placeholders(&template_dir).expect("Placeholder validation failed");
}

#[test]
fn test_conditional_syntax() {
    let template_dir = template_dir();
    validation::validate_conditional_syntax(&template_dir).expect("Conditional syntax validation failed");
}

#[test]
fn test_generate_simple_periodic() {
    let template_dir = template_dir();
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create workspace
    let workspace_dir = temp_dir.path().join("workspace");
    fs::create_dir_all(&workspace_dir).expect("Failed to create workspace");
    
    // Generate module
    let module_path = generation::generate_module(
        &template_dir,
        &workspace_dir,
        "test-simple",
        generation::ModuleType::SimplePeriodic,
    ).expect("Failed to generate module");
    
    // Validate structure
    generation::validate_simple_periodic(&module_path).expect("Simple periodic validation failed");
    
    // temp_dir automatically cleaned up on drop
}

#[test]
fn test_generate_http_fetcher() {
    let template_dir = template_dir();
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create workspace
    let workspace_dir = temp_dir.path().join("workspace");
    fs::create_dir_all(&workspace_dir).expect("Failed to create workspace");
    
    // Generate module
    let module_path = generation::generate_module(
        &template_dir,
        &workspace_dir,
        "test-fetcher",
        generation::ModuleType::HttpFetcher,
    ).expect("Failed to generate module");
    
    // Validate structure
    generation::validate_http_fetcher(&module_path).expect("HTTP fetcher validation failed");
    
    // temp_dir automatically cleaned up on drop
}

#[test]
fn test_generate_custom() {
    let template_dir = template_dir();
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create workspace
    let workspace_dir = temp_dir.path().join("workspace");
    fs::create_dir_all(&workspace_dir).expect("Failed to create workspace");
    
    // Generate module
    let module_path = generation::generate_module(
        &template_dir,
        &workspace_dir,
        "test-custom",
        generation::ModuleType::Custom,
    ).expect("Failed to generate module");
    
    // Validate structure
    generation::validate_custom(&module_path).expect("Custom validation failed");
    
    // temp_dir automatically cleaned up on drop
}
