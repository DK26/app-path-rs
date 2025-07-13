use crate::AppPath;
use std::env;
use std::fs;

#[test]
fn test_new_vs_old_methods_comparison() {
    let temp_dir = env::temp_dir().join("app_path_test_new_vs_old_methods");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test the difference between create_parents and create_dir
    let path = AppPath::new(temp_dir.join("testdir"));

    // Using create_parents - treats path as file, creates parent
    path.create_parents().unwrap();
    assert!(temp_dir.exists()); // Parent exists
    assert!(!path.exists()); // Path itself doesn't exist

    // Now using create_dir - creates the path as directory
    path.create_dir().unwrap();
    assert!(path.exists()); // Now the path exists
    assert!(path.is_dir()); // And it's a directory

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
#[allow(deprecated)]
fn test_deprecated_vs_new_methods_equivalence() {
    let temp_dir = env::temp_dir().join("app_path_test_deprecated_vs_new");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: ensure_parent_dirs vs create_parents should be equivalent
    let file_path_old = AppPath::new(temp_dir.join("old/logs/app.log"));
    let file_path_new = AppPath::new(temp_dir.join("new/logs/app.log"));

    file_path_old.ensure_parent_dirs().unwrap();
    file_path_new.create_parents().unwrap();

    // Both should create parent directories
    assert!(temp_dir.join("old/logs").exists());
    assert!(temp_dir.join("old/logs").is_dir());
    assert!(temp_dir.join("new/logs").exists());
    assert!(temp_dir.join("new/logs").is_dir());

    // Neither should create the file itself
    assert!(!file_path_old.exists());
    assert!(!file_path_new.exists());

    // Test 2: ensure_dir_exists vs create_dir should be equivalent
    let dir_path_old = AppPath::new(temp_dir.join("old/cache"));
    let dir_path_new = AppPath::new(temp_dir.join("new/cache"));

    dir_path_old.ensure_dir_exists().unwrap();
    dir_path_new.create_dir().unwrap();

    // Both should create the directory
    assert!(dir_path_old.exists());
    assert!(dir_path_old.is_dir());
    assert!(dir_path_new.exists());
    assert!(dir_path_new.is_dir());

    // Test 3: Nested directory creation
    let nested_dir_old = AppPath::new(temp_dir.join("old/data/backups/daily"));
    let nested_dir_new = AppPath::new(temp_dir.join("new/data/backups/daily"));

    nested_dir_old.ensure_dir_exists().unwrap();
    nested_dir_new.create_dir().unwrap();

    // Both should create all nested directories
    assert!(nested_dir_old.exists());
    assert!(nested_dir_old.is_dir());
    assert!(nested_dir_new.exists());
    assert!(nested_dir_new.is_dir());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
#[allow(deprecated)]
fn test_deprecated_create_dir_all_vs_new_methods() {
    let temp_dir = env::temp_dir().join("app_path_test_create_dir_all_vs_new");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test equivalence: create_dir_all should behave like create_parents for file paths
    let file_old = AppPath::new(temp_dir.join("old/config/app.toml"));
    let file_new = AppPath::new(temp_dir.join("new/config/app.toml"));

    file_old.create_dir_all().unwrap();
    file_new.create_parents().unwrap();

    // Both should create parent directories
    assert!(temp_dir.join("old/config").exists());
    assert!(temp_dir.join("old/config").is_dir());
    assert!(temp_dir.join("new/config").exists());
    assert!(temp_dir.join("new/config").is_dir());

    // Neither should create the file itself
    assert!(!file_old.exists());
    assert!(!file_new.exists());

    // Test with directory-like paths (no extension)
    let dir_old = AppPath::new(temp_dir.join("old/cache"));
    let dir_new_wrong = AppPath::new(temp_dir.join("new_wrong/cache"));
    let dir_new_correct = AppPath::new(temp_dir.join("new_correct/cache"));

    dir_old.create_dir_all().unwrap(); // Creates parent, not the path itself
    dir_new_wrong.create_parents().unwrap(); // Same behavior as create_dir_all
    dir_new_correct.create_dir().unwrap(); // Correct new method for directories

    // create_dir_all and create_parents should behave the same (create parent only)
    assert!(temp_dir.join("old").exists());
    assert!(!dir_old.exists()); // cache directory should not exist
    assert!(temp_dir.join("new_wrong").exists());
    assert!(!dir_new_wrong.exists()); // cache directory should not exist

    // create_dir should actually create the directory
    assert!(temp_dir.join("new_correct").exists());
    assert!(dir_new_correct.exists()); // cache directory SHOULD exist
    assert!(dir_new_correct.is_dir());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}
