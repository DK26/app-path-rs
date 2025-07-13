use crate::AppPath;
use std::env;
use std::fs;

#[test]
#[allow(deprecated)]
fn test_deprecated_ensure_parent_dirs() {
    let temp_dir = env::temp_dir().join("app_path_test_deprecated_ensure_parent_dirs");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test that deprecated method still works
    let file_path = AppPath::new(temp_dir.join("logs/app.log"));
    file_path.ensure_parent_dirs().unwrap();

    // Parent directory should exist
    assert!(temp_dir.join("logs").exists());
    assert!(temp_dir.join("logs").is_dir());
    // File should not exist (only parent created)
    assert!(!file_path.exists());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
#[allow(deprecated)]
fn test_deprecated_ensure_dir_exists() {
    let temp_dir = env::temp_dir().join("app_path_test_deprecated_ensure_dir_exists");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test that deprecated method still works
    let cache_dir = AppPath::new(temp_dir.join("cache"));
    cache_dir.ensure_dir_exists().unwrap();

    // Directory should exist
    assert!(cache_dir.exists());
    assert!(cache_dir.is_dir());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
#[allow(deprecated)]
fn test_deprecated_create_dir_all() {
    let temp_dir = env::temp_dir().join("app_path_test_deprecated_create_dir_all");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: File path - should create parent directories (not the file itself)
    let file_path = AppPath::new(temp_dir.join("logs/app.log"));
    file_path.create_dir_all().unwrap();

    // Parent directory should exist, but file should not
    assert!(temp_dir.join("logs").exists());
    assert!(temp_dir.join("logs").is_dir());
    assert!(!file_path.exists()); // File itself should not exist

    // Test 2: Directory-like path (no extension) - creates parent directories
    let dir_like_path = AppPath::new(temp_dir.join("cache/images"));
    dir_like_path.create_dir_all().unwrap();

    // Parent directory should exist
    assert!(temp_dir.join("cache").exists());
    assert!(temp_dir.join("cache").is_dir());
    // The path itself should not exist (treated as file)
    assert!(!dir_like_path.exists());

    // Test 3: Root-like path - should not error
    let root_path = AppPath::new(temp_dir.join("config.toml"));
    root_path.create_dir_all().unwrap(); // Should not error
    assert!(temp_dir.exists());
    assert!(!root_path.exists()); // File should not exist

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}
