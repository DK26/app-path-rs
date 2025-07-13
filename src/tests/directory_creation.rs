use crate::AppPath;
use std::env;
use std::fs;

#[test]
fn test_create_parents() {
    let temp_dir = env::temp_dir().join("app_path_test_create_parents");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: Basic file path - should create parent directories
    let file_path = AppPath::new(temp_dir.join("logs/app.log"));
    file_path.create_parents().unwrap();

    // Parent directory should exist
    assert!(temp_dir.join("logs").exists());
    assert!(temp_dir.join("logs").is_dir());
    // File should not exist (only parent created)
    assert!(!file_path.exists());

    // Test 2: Nested file path
    let nested_file = AppPath::new(temp_dir.join("data/2024/users.db"));
    nested_file.create_parents().unwrap();

    // All parent directories should exist
    assert!(temp_dir.join("data").exists());
    assert!(temp_dir.join("data/2024").exists());
    assert!(temp_dir.join("data/2024").is_dir());
    // File should not exist
    assert!(!nested_file.exists());

    // Test 3: File with no parent (root level in temp_dir)
    let root_file = AppPath::new(temp_dir.join("root.txt"));
    root_file.create_parents().unwrap(); // Should not error

    // temp_dir should exist (it's the parent)
    assert!(temp_dir.exists());
    assert!(!root_file.exists());

    // Test 4: File where parent already exists
    let existing_parent_file = AppPath::new(temp_dir.join("logs/another.log"));
    existing_parent_file.create_parents().unwrap(); // Should not error
    assert!(temp_dir.join("logs").exists());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_create_dir() {
    let temp_dir = env::temp_dir().join("app_path_test_create_dir");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: Basic directory creation
    let cache_dir = AppPath::new(temp_dir.join("cache"));
    cache_dir.create_dir().unwrap();

    // Directory should exist
    assert!(cache_dir.exists());
    assert!(cache_dir.is_dir());

    // Test 2: Nested directory creation
    let nested_dir = AppPath::new(temp_dir.join("data/backups/daily"));
    nested_dir.create_dir().unwrap();

    // All directories should exist
    assert!(temp_dir.join("data").exists());
    assert!(temp_dir.join("data/backups").exists());
    assert!(nested_dir.exists());
    assert!(nested_dir.is_dir());

    // Test 3: Directory that already exists (should not error)
    cache_dir.create_dir().unwrap(); // Should not error
    assert!(cache_dir.exists());
    assert!(cache_dir.is_dir());

    // Test 4: Directory with file-like name (has extension)
    let file_like_dir = AppPath::new(temp_dir.join("weird.txt"));
    file_like_dir.create_dir().unwrap();
    assert!(file_like_dir.exists());
    assert!(file_like_dir.is_dir()); // Should be a directory, not a file

    // Test 5: Directory creation where parent doesn't exist
    let orphan_dir = AppPath::new(temp_dir.join("missing/child"));
    orphan_dir.create_dir().unwrap();
    assert!(temp_dir.join("missing").exists());
    assert!(orphan_dir.exists());
    assert!(orphan_dir.is_dir());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

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
fn test_new_directory_creation_methods() {
    let temp_dir = env::temp_dir().join("app_path_test_new_methods");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: File path - should create parent directories
    let file_path = AppPath::new(temp_dir.join("logs/app.log"));
    file_path.create_parents().unwrap();

    // Parent directory should exist, but file should not
    assert!(temp_dir.join("logs").exists());
    assert!(temp_dir.join("logs").is_dir());
    assert!(!file_path.exists()); // File itself should not exist

    // Test 2: Directory path (no extension) - create directory using new method
    let dir_path = AppPath::new(temp_dir.join("data"));
    dir_path.create_dir().unwrap();

    // Directory should exist
    assert!(dir_path.exists());
    assert!(dir_path.is_dir());

    // Test 3: Nested directory path - create using new method
    let nested_dir = AppPath::new(temp_dir.join("cache/images"));
    nested_dir.create_dir().unwrap();

    // All levels should exist
    assert!(temp_dir.join("cache").exists());
    assert!(temp_dir.join("cache").is_dir());
    assert!(nested_dir.exists());
    assert!(nested_dir.is_dir());

    // Test 4: Existing directory - should not error
    let existing_dir = AppPath::new(temp_dir.join("data"));
    existing_dir.create_dir().unwrap(); // Should not error

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_create_dir_all_file_extensions() {
    let temp_dir = env::temp_dir().join("app_path_test_extensions");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test various file extensions - should create parent directories
    let extensions = vec!["txt", "log", "json", "toml", "yml", "db"];

    for ext in extensions {
        let file_path = AppPath::new(temp_dir.join(format!("files/test.{ext}")));
        file_path.create_parents().unwrap();

        // Parent directory should exist
        assert!(temp_dir.join("files").exists());
        assert!(temp_dir.join("files").is_dir());
        // File should not exist
        assert!(!file_path.exists());
    }

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_create_dir_all_edge_cases() {
    let temp_dir = env::temp_dir().join("app_path_test_edge_cases");
    let _ = fs::remove_dir_all(&temp_dir);

    // Test 1: Path with no extension (non-existent) - treated as file
    let no_ext_path = AppPath::new(temp_dir.join("extensionless_file"));
    no_ext_path.create_parents().unwrap();
    // Parent directory should exist
    assert!(temp_dir.exists());
    // The path itself should not exist (treated as file)
    assert!(!no_ext_path.exists());

    // Test 1b: Use new method for explicit directory creation
    let no_ext_dir = AppPath::new(temp_dir.join("node_modules"));
    no_ext_dir.create_dir().unwrap();
    assert!(no_ext_dir.exists());
    assert!(no_ext_dir.is_dir());

    // Test 2: Path with unusual extension (should be treated as file)
    let unusual_file = AppPath::new(temp_dir.join("backup/myfile.special"));
    unusual_file.create_parents().unwrap();
    assert!(temp_dir.join("backup").exists());
    assert!(temp_dir.join("backup").is_dir());
    assert!(!unusual_file.exists()); // File should not exist, only parent

    // Test 3: File with multiple extensions (should be treated as file)
    let multi_ext_file = AppPath::new(temp_dir.join("archives/file.tar.gz"));
    multi_ext_file.create_parents().unwrap();
    assert!(temp_dir.join("archives").exists());
    assert!(temp_dir.join("archives").is_dir());
    assert!(!multi_ext_file.exists());

    // Test 4: Root-level file (no parent to create)
    let root_file = AppPath::new(temp_dir.join("root.txt"));
    root_file.create_parents().unwrap(); // Should not error

    // Test 5: Attempting to create directory when file exists with same name
    let conflict_path = temp_dir.join("conflict.txt");
    fs::create_dir_all(&temp_dir).unwrap();
    fs::write(&conflict_path, "content").unwrap();

    let conflict_apppath = AppPath::new(&conflict_path);
    // Since conflict.txt has extension, it's treated as file, so create_parents
    // will try to create parent (temp_dir) which already exists, so it succeeds
    assert!(conflict_apppath.create_parents().is_ok());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_create_dir_all_preserves_existing_behavior() {
    let temp_dir = env::temp_dir().join("app_path_test_backward_compat");
    let _ = fs::remove_dir_all(&temp_dir);

    // This test ensures that code that worked before still works
    let deep_file = AppPath::new(temp_dir.join("deep/nested/dir/file.txt"));
    deep_file.create_parents().unwrap();

    // All parent directories should exist
    assert!(temp_dir.join("deep").exists());
    assert!(temp_dir.join("deep/nested").exists());
    assert!(temp_dir.join("deep/nested/dir").exists());

    // File should not exist (only parents were created)
    assert!(!deep_file.exists());

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

// === Tests for deprecated methods (ensure they still work) ===

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
