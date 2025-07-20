use crate::{AppPath, AppPathError};
use std::fmt::Write;

#[test]
fn test_error_type_display() {
    // Test that error types have meaningful Display implementations with realistic scenarios
    let exec_error =
        AppPathError::ExecutableNotFound("Failed to determine executable location".to_string());
    let invalid_error = AppPathError::InvalidExecutablePath(
        "Library file is not a valid executable path".to_string(),
    );

    let mut exec_str = String::new();
    write!(&mut exec_str, "{exec_error}").unwrap();
    assert!(exec_str.contains("Failed to determine executable location"));

    let mut invalid_str = String::new();
    write!(&mut invalid_str, "{invalid_error}").unwrap();
    assert!(invalid_str.contains("Invalid executable path"));
}

#[test]
fn test_error_type_debug() {
    // Test that error types have Debug implementations with realistic errors
    let exec_error =
        AppPathError::ExecutableNotFound("Cannot access current executable".to_string());
    let invalid_error =
        AppPathError::InvalidExecutablePath("Dynamic library is not an executable".to_string());

    let exec_debug = format!("{exec_error:?}");
    let invalid_debug = format!("{invalid_error:?}");

    assert!(exec_debug.contains("ExecutableNotFound"));
    assert!(invalid_debug.contains("InvalidExecutablePath"));
}

#[test]
fn test_error_type_equality() {
    // Test that error types implement PartialEq and Eq with realistic scenarios
    let error1 = AppPathError::ExecutableNotFound("Current executable access failed".to_string());
    let error2 = AppPathError::ExecutableNotFound("Current executable access failed".to_string());
    let error3 = AppPathError::ExecutableNotFound("Failed to access executable file".to_string());
    let error4 =
        AppPathError::InvalidExecutablePath("Library file is not a valid executable".to_string());

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
    assert_ne!(error1, error4);
}

#[test]
fn test_error_is_std_error() {
    // Test that our error type implements std::error::Error with realistic scenario
    let error =
        AppPathError::ExecutableNotFound("Failed to determine executable location".to_string());
    let _std_error: &dyn std::error::Error = &error;

    // Should compile without issues
}

#[test]
fn test_fallible_api_documentation_examples() {
    // Test the examples from the documentation work correctly

    // Example 1: Basic error handling pattern
    match AppPath::try_with("config.toml") {
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
        let config = AppPath::try_with("config.toml")?;
        Ok(config)
    }

    let config = load_config().unwrap();
    assert!(config.path().ends_with("config.toml"));

    // Example 3: Fallback strategy
    fn get_config_with_fallback() -> AppPath {
        AppPath::try_with("config.toml").unwrap_or_else(|_| {
            let temp_config = std::env::temp_dir().join("myapp").join("config.toml");
            AppPath::with(temp_config)
        })
    }

    let config = get_config_with_fallback();
    // Should succeed in either case
    assert!(config.path().is_absolute());
}

#[test]
fn test_io_error_variant_from_real_operation() {
    // Test conversion from a real I/O error by trying to read a non-existent file
    let result = std::fs::File::open("definitely_does_not_exist_12345.txt");

    match result {
        Err(io_error) => {
            let app_error = AppPathError::from(io_error);
            match app_error {
                AppPathError::IoError(msg) => {
                    // The error message will naturally be OS-appropriate
                    assert!(!msg.is_empty());
                }
                _ => panic!("Expected IoError variant"),
            }
        }
        Ok(_) => panic!("Expected file not found error"),
    }
}

#[test]
fn test_io_error_display_from_real_operation() {
    // Test with a real "directory not found" error by trying to create a directory in a non-existent parent
    let nonexistent_parent = std::env::temp_dir()
        .join("definitely_nonexistent_parent_12345")
        .join("child");
    let result = std::fs::create_dir(&nonexistent_parent);

    match result {
        Err(io_error) => {
            let app_error = AppPathError::from(io_error);
            let error_str = format!("{app_error}");
            assert!(error_str.contains("I/O operation failed"));
            // Don't check for specific text - let the OS provide its natural error message
        }
        Ok(_) => panic!("Expected directory creation to fail"),
    }
}

#[test]
fn test_io_error_debug_from_real_operation() {
    // Test with a real error by trying to open a directory as a file
    let temp_dir = std::env::temp_dir().join("app_path_debug_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let result = std::fs::File::open(&temp_dir);

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();

    match result {
        Err(io_error) => {
            let app_error = AppPathError::from(io_error);
            let debug_str = format!("{app_error:?}");
            assert!(debug_str.contains("IoError"));
            // The actual error message will be OS-appropriate naturally
        }
        Ok(_) => {
            // Some systems might allow opening a directory as a file, that's OK
            // Just verify the conversion would work
            let fake_error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "test");
            let app_error = AppPathError::from(fake_error);
            let debug_str = format!("{app_error:?}");
            assert!(debug_str.contains("IoError"));
        }
    }
}

#[cfg(unix)]
#[test]
fn test_create_parents_permission_error() {
    use std::os::unix::fs::PermissionsExt;

    // Create a test directory that we'll make read-only
    let temp_dir = std::env::temp_dir().join("app_path_permission_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Make it read-only (no write permissions)
    let mut perms = std::fs::metadata(&temp_dir).unwrap().permissions();
    perms.set_mode(0o444); // Read-only
    std::fs::set_permissions(&temp_dir, perms).unwrap();

    // Try to create a subdirectory (should fail with permission error)
    let protected_file = AppPath::with(temp_dir.join("protected/file.txt"));
    let result = protected_file.create_parents();

    // Restore write permissions for cleanup
    let mut perms = std::fs::metadata(&temp_dir).unwrap().permissions();
    perms.set_mode(0o755); // Restore write permissions
    std::fs::set_permissions(&temp_dir, perms).unwrap();

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();

    // Check that we got an IoError
    match result {
        Err(AppPathError::IoError(msg)) => {
            assert!(msg.contains("Permission denied") || msg.contains("Access is denied"));
        }
        _ => panic!("Expected IoError for permission denied, got: {result:?}"),
    }
}

#[cfg(unix)]
#[test]
fn test_create_dir_permission_error() {
    use std::os::unix::fs::PermissionsExt;

    // Create a test directory that we'll make read-only
    let temp_dir = std::env::temp_dir().join("app_path_dir_permission_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Make it read-only (no write permissions)
    let mut perms = std::fs::metadata(&temp_dir).unwrap().permissions();
    perms.set_mode(0o444); // Read-only
    std::fs::set_permissions(&temp_dir, perms).unwrap();

    // Try to create a subdirectory (should fail with permission error)
    let protected_dir = AppPath::with(temp_dir.join("protected"));
    let result = protected_dir.create_dir();

    // Restore write permissions for cleanup
    let mut perms = std::fs::metadata(&temp_dir).unwrap().permissions();
    perms.set_mode(0o755); // Restore write permissions
    std::fs::set_permissions(&temp_dir, perms).unwrap();

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();

    // Check that we got an IoError
    match result {
        Err(AppPathError::IoError(msg)) => {
            assert!(msg.contains("Permission denied") || msg.contains("Access is denied"));
        }
        _ => panic!("Expected IoError for permission denied, got: {result:?}"),
    }
}

#[test]
fn test_error_variant_completeness() {
    // Test all error variants for completeness
    let exec_error = AppPathError::ExecutableNotFound("exec error".to_string());
    let invalid_error = AppPathError::InvalidExecutablePath("invalid path".to_string());
    let io_error = AppPathError::IoError("io error".to_string());

    // Test Display
    assert!(format!("{exec_error}").contains("Failed to determine executable location"));
    assert!(format!("{invalid_error}").contains("Invalid executable path"));
    assert!(format!("{io_error}").contains("I/O operation failed"));

    // Test Debug
    assert!(format!("{exec_error:?}").contains("ExecutableNotFound"));
    assert!(format!("{invalid_error:?}").contains("InvalidExecutablePath"));
    assert!(format!("{io_error:?}").contains("IoError"));
}

#[test]
fn test_directory_creation_error_propagation() {
    // Test that directory creation methods properly propagate IoError

    // Create a file where we'll try to create a directory
    let temp_dir = std::env::temp_dir().join("app_path_error_propagation_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let blocking_file = temp_dir.join("blocking_file");
    std::fs::write(&blocking_file, "content").unwrap();

    // Try to create a directory with the same name as the file (should fail)
    let blocked_path = AppPath::from(&blocking_file);
    let result = blocked_path.create_dir();

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();

    // Should get an IoError
    match result {
        Err(AppPathError::IoError(_)) => {
            // Expected - trying to create a directory where a file exists
        }
        _ => panic!(
            "Expected IoError when trying to create directory over existing file, got: {result:?}"
        ),
    }
}

#[test]
fn test_create_parents_with_file_blocking_parent() {
    // Test create_parents when a file blocks parent creation

    let temp_dir = std::env::temp_dir().join("app_path_parent_block_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create a file that will block parent directory creation
    let blocking_file = temp_dir.join("logs");
    std::fs::write(&blocking_file, "content").unwrap();

    // Try to create parents for a path that needs "logs" as a directory
    let log_file = AppPath::with(temp_dir.join("logs/app.log"));
    let result = log_file.create_parents();

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();

    // Should get an IoError because "logs" exists as a file, not a directory
    match result {
        Err(AppPathError::IoError(_)) => {
            // Expected - can't create directory where file exists
        }
        _ => panic!("Expected IoError when file blocks parent creation, got: {result:?}"),
    }
}
