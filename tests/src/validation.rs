use std::path::Path;
use crate::{TestResult, file_exists, dir_exists};

/// Validate Init template structure
pub fn validate_init_template(template_dir: &Path) -> TestResult<()> {
    let init_dir = template_dir.join("Init");
    
    // Check required files
    assert!(file_exists(&init_dir.join("cargo-generate.toml")), "Init/cargo-generate.toml missing");
    assert!(file_exists(&init_dir.join("Cargo.toml")), "Init/Cargo.toml missing");
    assert!(dir_exists(&init_dir.join("modules/hello-world")), "hello-world module missing");
    
    Ok(())
}

/// Validate Modules template structure
pub fn validate_modules_template(template_dir: &Path) -> TestResult<()> {
    let modules_dir = template_dir.join("Modules");
    let bg_worker = modules_dir.join("background-worker");
    
    // Check required files
    assert!(file_exists(&modules_dir.join("cargo-generate.toml")), "Modules/cargo-generate.toml missing");
    assert!(dir_exists(&bg_worker.join("src")), "Module src directory missing");
    assert!(file_exists(&bg_worker.join("Cargo.toml")), "Module Cargo.toml missing");
    assert!(file_exists(&bg_worker.join("src/module.rs")), "module.rs missing");
    assert!(file_exists(&bg_worker.join("src/domain/mod.rs")), "domain/mod.rs missing");
    assert!(file_exists(&bg_worker.join("src/infra/mod.rs")), "infra/mod.rs missing");
    
    Ok(())
}

/// Validate placeholders in template files
pub fn validate_placeholders(template_dir: &Path) -> TestResult<()> {
    let bg_worker = template_dir.join("Modules/background-worker");
    
    // Check module_name placeholder
    let cargo_toml = std::fs::read_to_string(bg_worker.join("Cargo.toml"))?;
    assert!(cargo_toml.contains("{{module_name}}"), "module_name placeholder missing");
    
    // Check description placeholder
    let readme = std::fs::read_to_string(bg_worker.join("README.md"))?;
    assert!(readme.contains("{{description}}"), "description placeholder missing");
    
    // Check module_type conditional
    assert!(readme.contains("{% if module_type"), "module_type conditional missing");
    
    Ok(())
}

/// Validate conditional syntax in templates
pub fn validate_conditional_syntax(template_dir: &Path) -> TestResult<()> {
    let module_rs = std::fs::read_to_string(
        template_dir.join("Modules/background-worker/src/module.rs")
    )?;
    
    assert!(module_rs.contains("{% if module_type"), "Conditional logic missing");
    assert!(module_rs.contains("{% elsif"), "elsif syntax missing");
    assert!(module_rs.contains("{% endif %}"), "Closing tags missing");
    
    Ok(())
}
