use crate::{AppPath, AppPathError};
use std::path::PathBuf;

// === Error Trait Tests ===

#[test]
fn test_error_trait_display() {
    let error = AppPathError::ExecutableNotFound(
        "Failed to determine current executable location".to_string(),
    );
    let display_str = format!("{error}");
    assert!(display_str.contains("Failed to determine executable location"));
}

#[test]
fn test_error_trait_debug() {
    let error = AppPathError::ExecutableNotFound("Current executable access error".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("ExecutableNotFound"));
}

#[test]
fn test_error_trait_source() {
    let error = AppPathError::ExecutableNotFound("Cannot access current executable".to_string());
    // Test that Error trait is implemented (source() method)
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn test_error_trait_io_error_variant_from_real_operation() {
    // Test with a real permission error by trying to access a non-existent file
    let result = std::fs::metadata("definitely_does_not_exist_54321.txt");

    match result {
        Err(io_error) => {
            let error = AppPathError::from(io_error);
            let display_str = format!("{error}");
            assert!(display_str.contains("I/O operation failed"));

            let debug_str = format!("{error:?}");
            assert!(debug_str.contains("IoError"));
        }
        Ok(_) => panic!("Expected file not found error"),
    }
}

#[test]
fn test_error_trait_io_error_not_found_from_real_operation() {
    // Test with a real "file not found" error during directory operations
    let nonexistent_path = std::env::temp_dir()
        .join("nonexistent_12345")
        .join("also_nonexistent");
    let result = std::fs::remove_dir(&nonexistent_path);

    match result {
        Err(io_error) => {
            let error = AppPathError::from(io_error);
            let display_str = format!("{error}");
            assert!(display_str.contains("I/O operation failed"));

            let debug_str = format!("{error:?}");
            assert!(debug_str.contains("IoError"));
        }
        Ok(_) => panic!("Expected directory removal to fail"),
    }
}

#[test]
fn test_error_trait_source_for_io_error() {
    // IoError variant now preserves the original io::Error and implements source()
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let error = AppPathError::IoError(io_error);
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn test_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let app_error = AppPathError::from(io_error);

    assert!(matches!(app_error, AppPathError::IoError(_)));
    assert!(app_error.to_string().contains("file not found"));
}

#[test]
fn test_error_from_io_error_with_path() {
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
    let path = PathBuf::from("/test/path");
    let app_error = AppPathError::from((io_error, &path));

    assert!(matches!(app_error, AppPathError::IoError(_)));
    assert!(app_error.to_string().contains("permission denied"));
    assert!(app_error.to_string().contains("/test/path"));
}

// === Integration Tests with Standard Library ===

#[test]
fn test_works_with_std_functions() {
    let app_path = AppPath::with("test.toml");

    // Should work with functions expecting AsRef<Path>
    let metadata_result = std::fs::metadata(&app_path);
    // Don't assert success since file might not exist, just verify it compiles
    let _ = metadata_result;

    // Should work with PathBuf::join
    let joined = PathBuf::from("/tmp").join(&app_path);
    assert!(joined.ends_with("test.toml"));
}

#[test]
fn test_collection_operations() {
    let paths = [
        AppPath::with("a.toml"),
        AppPath::with("b.toml"),
        AppPath::with("c.toml"),
    ];

    // Should work with iterators
    let names: Vec<_> = paths.iter().map(|p| p.file_name().unwrap()).collect();

    assert_eq!(names.len(), 3);
    assert!(names.contains(&std::ffi::OsStr::new("a.toml")));
}

#[test]
fn test_borrow_checker_friendly() {
    let app_path = AppPath::with("config.toml");

    // Should be able to borrow and move without issues
    let borrowed_ref = &app_path;
    let file_name = borrowed_ref.file_name();

    // Original should still be usable
    let extension = app_path.extension();

    assert!(file_name.is_some());
    assert!(extension.is_some());
}
