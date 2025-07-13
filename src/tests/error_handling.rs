use crate::{AppPath, AppPathError};
use std::fmt::Write;

#[test]
fn test_error_type_display() {
    // Test that error types have meaningful Display implementations
    let exec_error = AppPathError::ExecutableNotFound("test error".to_string());
    let invalid_error = AppPathError::InvalidExecutablePath("test path".to_string());

    let mut exec_str = String::new();
    write!(&mut exec_str, "{exec_error}").unwrap();
    assert!(exec_str.contains("Failed to determine executable location"));
    assert!(exec_str.contains("test error"));

    let mut invalid_str = String::new();
    write!(&mut invalid_str, "{invalid_error}").unwrap();
    assert!(invalid_str.contains("Invalid executable path"));
    assert!(invalid_str.contains("test path"));
}

#[test]
fn test_error_type_debug() {
    // Test that error types have Debug implementations
    let exec_error = AppPathError::ExecutableNotFound("test".to_string());
    let invalid_error = AppPathError::InvalidExecutablePath("test".to_string());

    let exec_debug = format!("{exec_error:?}");
    let invalid_debug = format!("{invalid_error:?}");

    assert!(exec_debug.contains("ExecutableNotFound"));
    assert!(invalid_debug.contains("InvalidExecutablePath"));
}

#[test]
fn test_error_type_equality() {
    // Test that error types implement PartialEq and Eq
    let error1 = AppPathError::ExecutableNotFound("same".to_string());
    let error2 = AppPathError::ExecutableNotFound("same".to_string());
    let error3 = AppPathError::ExecutableNotFound("different".to_string());
    let error4 = AppPathError::InvalidExecutablePath("path".to_string());

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
    assert_ne!(error1, error4);
}

#[test]
fn test_error_is_std_error() {
    // Test that our error type implements std::error::Error
    let error = AppPathError::ExecutableNotFound("test".to_string());
    let _std_error: &dyn std::error::Error = &error;

    // Should compile without issues
}

#[test]
fn test_fallible_api_documentation_examples() {
    // Test the examples from the documentation work correctly

    // Example 1: Basic error handling pattern
    match AppPath::try_new("config.toml") {
        Ok(config) => {
            assert!(config.path().ends_with("config.toml"));
        }
        Err(_e) => {
            // In our test environment, this shouldn't happen
            panic!("try_new should succeed in test environment");
        }
    }

    // Example 2: Using ? operator (simulated)
    fn load_config() -> Result<AppPath, AppPathError> {
        let config = AppPath::try_new("config.toml")?;
        Ok(config)
    }

    let config = load_config().unwrap();
    assert!(config.path().ends_with("config.toml"));

    // Example 3: Fallback strategy
    fn get_config_with_fallback() -> AppPath {
        AppPath::try_new("config.toml").unwrap_or_else(|_| {
            let temp_config = std::env::temp_dir().join("myapp").join("config.toml");
            AppPath::new(temp_config)
        })
    }

    let config = get_config_with_fallback();
    // Should succeed in either case
    assert!(config.path().is_absolute());
}
