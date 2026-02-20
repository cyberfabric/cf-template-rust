use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use anyhow::Result;
use cargo_generate::{GenerateArgs, TemplatePath, generate};
use crate::{file_exists, dir_exists};

/// Module type for template generation
#[derive(Debug, Clone, Copy)]
pub enum ModuleType {
    SimplePeriodic,
    HttpFetcher,
    Custom,
}

impl ModuleType {
    pub fn as_str(&self) -> &str {
        match self {
            ModuleType::SimplePeriodic => "simple-periodic",
            ModuleType::HttpFetcher => "http-fetcher",
            ModuleType::Custom => "custom",
        }
    }
}

/// Generate a module from template using cargo-generate library
pub fn generate_module(
    template_dir: &Path,
    output_dir: &Path,
    module_name: &str,
    module_type: ModuleType,
) -> Result<PathBuf> {
    let module_path = output_dir.join(module_name);
    
    // Remove existing directory if it exists
    if module_path.exists() {
        fs::remove_dir_all(&module_path)?;
    }
    
    // Create the module directory
    fs::create_dir_all(&module_path)?;
    
    // Prepare template variables in key=value format
    let define = vec![
        format!("module_name={}", module_name),
        format!("module_type={}", module_type.as_str()),
        "description=Test module".to_string(),
        "task_interval_secs=5".to_string(),
        "http_url=https://api.example.com".to_string(),
        "fetch_interval_secs=30".to_string(),
    ];
    
    // Use cargo-generate library
    let template_path = template_dir.join("Modules/background-worker");
    generate(GenerateArgs {
        template_path: TemplatePath {
            auto_path: None,
            git: None,
            path: Some(template_path.to_string_lossy().to_string()),
            subfolder: None,
            branch: None,
            tag: None,
            test: false,
            revision: None,
            favorite: None,
        },
        destination: Some(module_path.clone()),
        overwrite: false,
        init: true,
        name: Some(module_name.to_string()),
        quiet: true,
        verbose: false,
        force_git_init: false,
        lib: false,
        no_workspace: true,
        define,
        ..Default::default()
    })?;
    
    Ok(module_path)
}

/// Validate generated module structure for simple-periodic type
pub fn validate_simple_periodic(module_path: &Path) -> Result<()> {
    assert!(file_exists(&module_path.join("Cargo.toml")), "Cargo.toml missing");
    assert!(file_exists(&module_path.join("src/module.rs")), "module.rs missing");
    
    // Should NOT have domain/infra for simple-periodic
    assert!(!dir_exists(&module_path.join("src/domain")), "simple-periodic should not have domain directory");
    assert!(!dir_exists(&module_path.join("src/infra")), "simple-periodic should not have infra directory");
    
    // Check placeholders are replaced
    let cargo_toml = std::fs::read_to_string(module_path.join("Cargo.toml"))?;
    assert!(!cargo_toml.contains("{{module_name}}"), "module_name placeholder not replaced");
    
    let module_rs = std::fs::read_to_string(module_path.join("src/module.rs"))?;
    assert!(!module_rs.contains("{{task_interval_secs}}"), "task_interval_secs placeholder not replaced");
    
    // Validate SDK
    validate_sdk(module_path)?;
    
    Ok(())
}

/// Validate generated module structure for http-fetcher type
pub fn validate_http_fetcher(module_path: &Path) -> Result<()> {
    assert!(file_exists(&module_path.join("Cargo.toml")), "Cargo.toml missing");
    assert!(file_exists(&module_path.join("src/module.rs")), "module.rs missing");
    
    // SHOULD have domain/infra for http-fetcher
    assert!(dir_exists(&module_path.join("src/domain")), "http-fetcher should have domain directory");
    assert!(dir_exists(&module_path.join("src/infra")), "http-fetcher should have infra directory");
    
    // SHOULD have dto.rs in infra for http-fetcher
    assert!(file_exists(&module_path.join("src/infra/dto.rs")), "http-fetcher should have infra/dto.rs");
    
    // Check HTTP dependencies
    let cargo_toml = std::fs::read_to_string(module_path.join("Cargo.toml"))?;
    assert!(cargo_toml.contains("cf-modkit-http"), "cf-modkit-http dependency missing");
    
    // Check placeholders are replaced
    let infra_mod = std::fs::read_to_string(module_path.join("src/infra/mod.rs"))?;
    assert!(!infra_mod.contains("{{http_url}}"), "http_url placeholder not replaced");
    
    // Verify domain model doesn't have Serialize/Deserialize
    let domain_mod = std::fs::read_to_string(module_path.join("src/domain/mod.rs"))?;
    assert!(!domain_mod.contains("use serde"), "domain model should not import serde");
    assert!(!domain_mod.contains("Serialize, Deserialize"), "domain model should not derive Serialize/Deserialize");
    
    // Verify DTO has Serialize/Deserialize
    let dto_rs = std::fs::read_to_string(module_path.join("src/infra/dto.rs"))?;
    assert!(dto_rs.contains("Serialize, Deserialize"), "DTO should derive Serialize/Deserialize");
    
    // Validate SDK
    validate_sdk(module_path)?;
    
    Ok(())
}

/// Validate generated module structure for custom type
pub fn validate_custom(module_path: &Path) -> Result<()> {
    assert!(file_exists(&module_path.join("Cargo.toml")), "Cargo.toml missing");
    assert!(file_exists(&module_path.join("src/module.rs")), "module.rs missing");
    
    // Should NOT have domain/infra for custom
    assert!(!dir_exists(&module_path.join("src/domain")), "custom should not have domain directory");
    assert!(!dir_exists(&module_path.join("src/infra")), "custom should not have infra directory");
    
    // Validate SDK
    validate_sdk(module_path)?;
    
    Ok(())
}

/// Validate SDK stub generation
fn validate_sdk(module_path: &Path) -> Result<()> {
    // SDK should be in sdk subdirectory within the module
    let sdk_path = module_path.join("sdk");
    
    assert!(dir_exists(&sdk_path), "SDK directory should exist at {:?}", sdk_path);
    assert!(file_exists(&sdk_path.join("Cargo.toml")), "SDK Cargo.toml missing");
    assert!(file_exists(&sdk_path.join("src/lib.rs")), "SDK lib.rs missing");
    assert!(file_exists(&sdk_path.join("README.md")), "SDK README.md missing");
    
    // Check SDK has client trait
    let lib_rs = std::fs::read_to_string(sdk_path.join("src/lib.rs"))?;
    assert!(lib_rs.contains("Client"), "SDK should define a Client trait");
    assert!(lib_rs.contains("#[modkit::async_trait]"), "SDK client should use async_trait");
    
    // Check placeholders are replaced
    assert!(!lib_rs.contains("{{module_name}}"), "SDK module_name placeholder not replaced");
    
    Ok(())
}

/// Build a generated module to verify it compiles
pub fn build_module(workspace_dir: &Path, module_name: &str) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("check")
        .arg("-p")
        .arg(module_name)
        .current_dir(workspace_dir);
    
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("Module {} failed to build", module_name);
    }
    
    Ok(())
}
