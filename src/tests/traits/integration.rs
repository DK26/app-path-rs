use crate::{AppPath, AppPathError};
use std::path::PathBuf;

// === Error Trait Tests ===

#[test]
fn test_error_trait_display() {
    let error = AppPathError::ExecutableNotFound("test error".to_string());
    let display_str = format!("{error}");
    assert!(display_str.contains("test error"));
}

#[test]
fn test_error_trait_debug() {
    let error = AppPathError::ExecutableNotFound("debug test".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("debug test"));
    assert!(debug_str.contains("ExecutableNotFound"));
}

#[test]
fn test_error_trait_source() {
    let error = AppPathError::ExecutableNotFound("source test".to_string());
    // Test that Error trait is implemented (source() method)
    assert!(std::error::Error::source(&error).is_none());
}

// === Integration Tests with Standard Library ===

#[test]
fn test_works_with_std_functions() {
    let app_path = AppPath::new("test.toml");

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
        AppPath::new("a.toml"),
        AppPath::new("b.toml"),
        AppPath::new("c.toml"),
    ];

    // Should work with iterators
    let names: Vec<_> = paths.iter().map(|p| p.file_name().unwrap()).collect();

    assert_eq!(names.len(), 3);
    assert!(names.contains(&std::ffi::OsStr::new("a.toml")));
}

#[test]
fn test_borrow_checker_friendly() {
    let app_path = AppPath::new("config.toml");

    // Should be able to borrow and move without issues
    let borrowed_ref = &app_path;
    let file_name = borrowed_ref.file_name();

    // Original should still be usable
    let extension = app_path.extension();

    assert!(file_name.is_some());
    assert!(extension.is_some());
}
