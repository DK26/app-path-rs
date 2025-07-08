use crate::{exe_dir, try_exe_dir, AppPath, AppPathError};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Helper to create a file at a given path for testing.
fn create_test_file(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    writeln!(file, "test").unwrap();
}

#[test]
fn resolves_relative_path_to_exe_dir() {
    let rel = "myconfig.toml";
    let rel_path = AppPath::new(rel);
    let expected = exe_dir().join(rel);

    assert_eq!(rel_path.path(), &expected);
    assert!(rel_path.path().is_absolute());
}

#[test]
fn resolves_relative_path_with_custom_base() {
    let temp_dir = env::temp_dir().join("app_path_test_base");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let rel = "subdir/file.txt";
    let rel_path = AppPath::new(temp_dir.join(rel));
    let expected = temp_dir.join(rel);

    assert_eq!(rel_path.path(), &expected);
    assert!(rel_path.path().is_absolute());
}

#[test]
fn can_access_file_using_full_path() {
    let temp_dir = env::temp_dir().join("app_path_test_access");
    let file_name = "access.txt";
    let file_path = temp_dir.join(file_name);
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    create_test_file(&file_path);

    let rel_path = AppPath::new(temp_dir.join(file_name));
    assert!(rel_path.exists());
    assert_eq!(rel_path.path(), &file_path);
}

#[test]
fn handles_dot_and_dotdot_components() {
    let temp_dir = env::temp_dir().join("app_path_test_dot");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let rel = "./foo/../bar.txt";
    let rel_path = AppPath::new(temp_dir.join(rel));
    let expected = temp_dir.join(rel);

    assert_eq!(rel_path.path(), &expected);
}

#[test]
fn as_ref_and_into_pathbuf_are_consistent() {
    let rel = "somefile.txt";
    let rel_path = AppPath::new(rel);
    let as_ref_path: &Path = rel_path.as_ref();
    let into_pathbuf: PathBuf = rel_path.clone().into();
    assert_eq!(as_ref_path, into_pathbuf.as_path());
}

#[test]
fn test_path_method() {
    let rel = "data/file.txt";
    let temp_dir = env::temp_dir().join("app_path_test_full");
    let rel_path = AppPath::new(temp_dir.join(rel));
    assert_eq!(rel_path.path(), temp_dir.join(rel));
}

#[test]
fn test_exists_method() {
    let temp_dir = env::temp_dir().join("app_path_test_exists");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let file_name = "exists_test.txt";
    let file_path = temp_dir.join(file_name);
    create_test_file(&file_path);

    let rel_path = AppPath::new(temp_dir.join(file_name));
    assert!(rel_path.exists());

    let non_existent = AppPath::new(temp_dir.join("non_existent.txt"));
    assert!(!non_existent.exists());
}

#[test]
fn test_create_dir_all() {
    let temp_dir = env::temp_dir().join("app_path_test_create");
    let _ = fs::remove_dir_all(&temp_dir);

    let rel = "deep/nested/dir/file.txt";
    let rel_path = AppPath::new(temp_dir.join(rel));

    rel_path.create_dir_all().unwrap();
    assert!(rel_path.path().parent().unwrap().exists());
}

#[test]
fn test_display_trait() {
    let rel = "display_test.txt";
    let temp_dir = env::temp_dir().join("app_path_test_display");
    let rel_path = AppPath::new(temp_dir.join(rel));

    let expected = temp_dir.join(rel);
    assert_eq!(format!("{rel_path}"), format!("{}", expected.display()));
}

#[test]
fn test_from_str() {
    let rel_path = AppPath::from("config.toml");
    let expected = exe_dir().join("config.toml");

    assert_eq!(rel_path.path(), &expected);
}

#[test]
fn test_from_string() {
    let path_string = "data/file.txt".to_string();
    let rel_path = AppPath::from(path_string);
    let expected = exe_dir().join("data/file.txt");

    assert_eq!(rel_path.path(), &expected);
}

#[test]
fn test_from_string_ref() {
    let path_string = "logs/app.log".to_string();
    let rel_path = AppPath::from(&path_string);
    let expected = exe_dir().join("logs/app.log");

    assert_eq!(rel_path.path(), &expected);
}

#[test]
fn test_new_with_different_types() {
    use std::path::PathBuf;

    // Test various input types with new
    let from_str = AppPath::new("test.txt");
    let test_string = "test.txt".to_string(); // Intentionally create String to test type
    let from_string = AppPath::new(test_string);
    let from_path_buf = AppPath::new(PathBuf::from("test.txt"));
    let from_path_ref = AppPath::new(Path::new("test.txt"));

    // All should produce equivalent results
    assert_eq!(from_str.path(), from_string.path());
    assert_eq!(from_string.path(), from_path_buf.path());
    assert_eq!(from_path_buf.path(), from_path_ref.path());
}

#[test]
fn test_ownership_transfer() {
    use std::path::PathBuf;

    let path_buf = PathBuf::from("test.txt");
    let app_path = AppPath::new(path_buf);
    // path_buf is moved and no longer accessible

    let expected = exe_dir().join("test.txt");
    assert_eq!(app_path.path(), &expected);

    // Test with String too
    let string_path = "another_test.txt".to_string();
    let app_path2 = AppPath::new(string_path);
    // string_path is moved and no longer accessible

    let expected2 = exe_dir().join("another_test.txt");
    assert_eq!(app_path2.path(), &expected2);
}

#[test]
fn test_absolute_path_behavior() {
    let absolute_path = if cfg!(windows) {
        r"C:\temp\config.toml"
    } else {
        "/tmp/config.toml"
    };

    let app_path = AppPath::new(absolute_path);

    // PathBuf::join() with absolute paths replaces the base path entirely
    assert_eq!(app_path.path(), Path::new(absolute_path));
    assert!(app_path.path().is_absolute());
}

#[test]
fn test_exe_dir_function() {
    let dir = exe_dir();
    assert!(dir.is_absolute());

    // Should be consistent with AppPath behavior
    let config = AppPath::new("test.txt");
    let expected = dir.join("test.txt");
    assert_eq!(config.path(), &expected);
}

#[test]
fn test_from_implementations() {
    use std::path::{Path, PathBuf};

    let expected = exe_dir().join("test.txt");

    // Test all From implementations
    let from_str: AppPath = "test.txt".into();
    let from_string: AppPath = "test.txt".to_string().into();
    let from_string_ref: AppPath = (&"test.txt".to_string()).into();
    let from_path: AppPath = Path::new("test.txt").into();
    let from_pathbuf: AppPath = PathBuf::from("test.txt").into();
    let from_pathbuf_ref: AppPath = (&PathBuf::from("test.txt")).into();

    // All should produce the same result
    assert_eq!(from_str.path(), &expected);
    assert_eq!(from_string.path(), &expected);
    assert_eq!(from_string_ref.path(), &expected);
    assert_eq!(from_path.path(), &expected);
    assert_eq!(from_pathbuf.path(), &expected);
    assert_eq!(from_pathbuf_ref.path(), &expected);
}

#[test]
fn test_asref_path_efficiency() {
    use std::path::{Path, PathBuf};

    // Test that AsRef<Path> works efficiently with various types
    let str_path = "test.txt";
    let string_path = "test.txt".to_string();
    let path_ref = Path::new("test.txt");
    let path_buf = PathBuf::from("test.txt");

    let from_str = AppPath::new(str_path);
    let from_string = AppPath::new(&string_path); // Reference to avoid move
    let from_path = AppPath::new(path_ref);
    let from_pathbuf = AppPath::new(&path_buf); // Reference to avoid move

    let expected = exe_dir().join("test.txt");

    assert_eq!(from_str.path(), &expected);
    assert_eq!(from_string.path(), &expected);
    assert_eq!(from_path.path(), &expected);
    assert_eq!(from_pathbuf.path(), &expected);

    // Verify original values are still accessible (weren't moved)
    assert_eq!(string_path, "test.txt");
    assert_eq!(path_buf, PathBuf::from("test.txt"));
}

#[test]
fn test_root_directory_edge_case() {
    // This test simulates the edge case where an executable might be at filesystem root
    // We can't easily test this in practice, but we can test the logic

    if cfg!(windows) {
        // Test Windows-style root
        let windows_root = Path::new(r"C:\");
        let config = AppPath::new(windows_root.join("config.toml"));
        let expected_windows = windows_root.join("config.toml");
        assert_eq!(config.path(), &expected_windows);
    } else {
        // Test Unix-style root
        let unix_root = Path::new("/");
        let config = AppPath::new(unix_root.join("config.toml"));
        let expected_unix = unix_root.join("config.toml");
        assert_eq!(config.path(), &expected_unix);
    }
}

#[test]
fn test_root_directory_behavior_with_absolute_paths() {
    // Test that absolute paths work correctly even when base is root
    let absolute_path = if cfg!(windows) {
        r"D:\temp\config.toml"
    } else {
        "/tmp/config.toml"
    };

    let app_path = AppPath::new(absolute_path);

    // Absolute paths should override the base entirely
    assert_eq!(app_path.path(), Path::new(absolute_path));
    assert!(app_path.path().is_absolute());
}

#[test]
fn test_root_directory_nested_paths() {
    // Test that nested relative paths work correctly from root
    let root = if cfg!(windows) {
        Path::new(r"C:\")
    } else {
        Path::new("/")
    };

    let nested_path = "app/data/config.toml";
    let app_path = AppPath::new(root.join(nested_path));

    let expected = if cfg!(windows) {
        Path::new(r"C:\app\data\config.toml")
    } else {
        Path::new("/app/data/config.toml")
    };

    assert_eq!(app_path.path(), expected);
    assert!(app_path.path().is_absolute());
}

#[test]
fn test_exe_dir_static_initialization() {
    // Test that exe_dir() works and returns an absolute path
    let dir = exe_dir();
    assert!(dir.is_absolute());

    // Test that it's consistent across multiple calls
    let dir2 = exe_dir();
    assert_eq!(dir, dir2);

    // Test that it works with AppPath
    let config = AppPath::new("test.txt");
    let expected = dir.join("test.txt");
    assert_eq!(config.path(), &expected);
}

#[test]
fn test_exe_dir_edge_case_simulation() {
    // We can't easily simulate the actual root directory edge case,
    // but we can test that our logic works correctly

    use std::path::PathBuf;

    // Simulate what would happen with a root-level executable
    let fake_root_exe = if cfg!(windows) {
        PathBuf::from(r"C:\app.exe")
    } else {
        PathBuf::from("/init")
    };

    // Test the logic that would be used in the actual edge case
    let parent = fake_root_exe.parent();
    let base_dir = match parent {
        Some(p) => p.to_path_buf(),
        None => {
            // This is the edge case logic from our implementation
            fake_root_exe.ancestors().last().unwrap().to_path_buf()
        }
    };

    let expected_root = if cfg!(windows) {
        PathBuf::from(r"C:\")
    } else {
        PathBuf::from("/")
    };

    assert_eq!(base_dir, expected_root);
}

#[test]
fn test_containerized_environment_simulation() {
    // Test behavior that might occur in containerized environments
    // where the executable could be at various root-like locations

    let container_roots = if cfg!(windows) {
        vec![r"C:\", r"D:\app"]
    } else {
        vec!["/", "/app", "/usr/bin"]
    };

    for root in container_roots {
        let root_path = Path::new(root);
        let config = AppPath::new(root_path.join("config.toml"));
        let data = AppPath::new(root_path.join("data/app.db"));

        // Paths should be properly resolved
        assert!(config.path().is_absolute());
        assert!(data.path().is_absolute());

        // Should maintain the root as prefix
        assert!(config.path().starts_with(root));
        assert!(data.path().starts_with(root));
    }
}

#[test]
fn test_jailed_environment_patterns() {
    // Test common patterns that might occur in jailed/chrooted environments
    let jail_root = if cfg!(windows) {
        r"C:\jail"
    } else {
        "/var/jail"
    };

    // Test that relative paths work correctly in jailed environments
    let jail_root_path = Path::new(jail_root);
    let config = AppPath::new(jail_root_path.join("etc/config.toml"));
    let data = AppPath::new(jail_root_path.join("var/data/app.db"));
    let logs = AppPath::new(jail_root_path.join("var/log/app.log"));

    // All paths should be absolute and start with the jail root
    assert!(config.path().is_absolute());
    assert!(data.path().is_absolute());
    assert!(logs.path().is_absolute());

    assert!(config.path().starts_with(jail_root));
    assert!(data.path().starts_with(jail_root));
    assert!(logs.path().starts_with(jail_root));
}

#[test]
fn test_panic_conditions_documentation() {
    // This test documents the conditions that would cause panics
    // It doesn't actually panic, but serves as documentation

    // These are the conditions that would cause the static initialization to panic:
    // 1. std::env::current_exe() fails
    // 2. The executable path is empty
    // 3. ancestors().last() fails (extremely rare)

    // We can't easily test these conditions in a unit test since they're
    // part of static initialization, but we can document them

    // The actual panic would happen during the first call to any AppPath function
    // or exe_dir() function when the OnceLock is initialized

    // For testing purposes, we just verify that normal operation works
    let _config = AppPath::new("config.toml");
    let _dir = exe_dir();

    // If we reach here, the static initialization succeeded
    // Test passes by reaching this point without panicking
}

// === Tests for Additional Trait Implementations ===

#[test]
fn test_default_implementation() {
    let default_path = AppPath::default();
    let empty_path = AppPath::new("");

    // Default should be equivalent to new("")
    assert_eq!(default_path, empty_path);

    // Default should point to executable directory
    assert_eq!(default_path.path(), exe_dir());
}

#[test]
fn test_equality_and_inequality() {
    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("config.toml");
    let path3 = AppPath::new("other.toml");
    let path4 = AppPath::new("subdir/config.toml");

    // Same paths should be equal
    assert_eq!(path1, path2);
    assert!(path1 == path2);

    // Different paths should not be equal
    assert_ne!(path1, path3);
    assert_ne!(path1, path4);
    assert!(path1 != path3);
    assert!(path1 != path4);
}

#[test]
fn test_ordering() {
    let path_a = AppPath::new("a.txt");
    let path_b = AppPath::new("b.txt");
    let path_z = AppPath::new("z.txt");
    let path_subdir = AppPath::new("subdir/a.txt");

    // Lexicographic ordering
    assert!(path_a < path_b);
    assert!(path_b < path_z);
    assert!(path_a < path_z);

    // With subdirectories (depends on path separator)
    // We just verify that ordering is consistent
    assert!(path_a.cmp(&path_subdir) != std::cmp::Ordering::Equal);

    // Verify partial_cmp is consistent with cmp
    assert_eq!(path_a.partial_cmp(&path_b), Some(path_a.cmp(&path_b)));
}

#[test]
fn test_sorting() {
    use std::collections::BTreeSet;

    let mut paths = [
        AppPath::new("z.txt"),
        AppPath::new("a.txt"),
        AppPath::new("m.txt"),
        AppPath::new("b.txt"),
    ];

    // Sort the vector
    paths.sort();

    // Should be in lexicographic order
    assert!(paths[0] <= paths[1]);
    assert!(paths[1] <= paths[2]);
    assert!(paths[2] <= paths[3]);

    // Test with BTreeSet (which uses Ord)
    let mut set = BTreeSet::new();
    set.insert(AppPath::new("z.txt"));
    set.insert(AppPath::new("a.txt"));
    set.insert(AppPath::new("m.txt"));

    let sorted: Vec<_> = set.into_iter().collect();
    // BTreeSet maintains sorted order
    assert!(sorted[0] <= sorted[1]);
    assert!(sorted[1] <= sorted[2]);
}

#[test]
fn test_hash_implementation() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::{HashMap, HashSet};
    use std::hash::{Hash, Hasher};

    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("config.toml");
    let path3 = AppPath::new("other.toml");

    // Equal paths should have equal hashes
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    path1.hash(&mut hasher1);
    path2.hash(&mut hasher2);
    assert_eq!(hasher1.finish(), hasher2.finish());

    // Test with HashMap
    let mut map = HashMap::new();
    map.insert(path1.clone(), "config data");
    map.insert(path3.clone(), "other data");

    // Should be able to look up the same paths
    assert_eq!(map.get(&path2), Some(&"config data"));
    assert_eq!(map.get(&path3), Some(&"other data"));

    // Test with HashSet
    let mut set = HashSet::new();
    set.insert(path1.clone());
    set.insert(path3.clone());

    assert!(set.contains(&path2)); // Equal to path1
    assert!(set.contains(&path3));
    assert_eq!(set.len(), 2); // path1 and path2 are the same
}

#[test]
fn test_deref_implementation() {
    let app_path = AppPath::new("config.toml");

    // Should be able to use Path methods directly
    assert_eq!(app_path.extension(), Some("toml".as_ref()));
    assert_eq!(app_path.file_name(), Some("config.toml".as_ref()));

    // Should work with functions expecting &Path
    fn get_file_name(path: &Path) -> Option<&std::ffi::OsStr> {
        path.file_name()
    }

    assert_eq!(get_file_name(&app_path), Some("config.toml".as_ref()));

    // Should be able to call Path methods through deref
    let stem = app_path.file_stem();
    assert_eq!(stem, Some("config".as_ref()));
}

#[test]
fn test_borrow_implementation() {
    use std::borrow::Borrow;

    let app_path = AppPath::new("config.toml");

    // Should be able to borrow as Path
    let borrowed: &Path = app_path.borrow();
    assert_eq!(borrowed, app_path.path()); // Test with function that accepts Borrow<Path>
    fn process_borrowed_path<P: Borrow<Path>>(path: P) -> Option<String> {
        let path_ref: &Path = path.borrow();
        path_ref.extension()?.to_str().map(|s| s.to_string())
    }

    assert_eq!(
        process_borrowed_path(app_path.clone()),
        Some("toml".to_string())
    );
    assert_eq!(
        process_borrowed_path(app_path.path()),
        Some("toml".to_string())
    );
}

// === Fallible API Tests ===

#[test]
fn test_try_new_success() {
    // try_new should work for all the same inputs as new()
    let config = AppPath::try_new("config.toml").unwrap();
    let expected = exe_dir().join("config.toml");
    assert_eq!(config.path(), &expected);

    let data = AppPath::try_new("data/users.db").unwrap();
    let expected = exe_dir().join("data/users.db");
    assert_eq!(data.path(), &expected);
}

#[test]
fn test_try_new_with_different_types() {
    use std::path::{Path, PathBuf};

    // Test all the same types that work with new()
    let from_str = AppPath::try_new("config.toml").unwrap();
    let from_string = AppPath::try_new("config.toml").unwrap();
    let from_string_ref = AppPath::try_new("config.toml").unwrap();
    let from_path = AppPath::try_new(Path::new("config.toml")).unwrap();
    let from_pathbuf = AppPath::try_new(PathBuf::from("config.toml")).unwrap();
    let from_pathbuf_ref = AppPath::try_new(PathBuf::from("config.toml")).unwrap();

    // All should resolve to the same path
    let expected = exe_dir().join("config.toml");
    assert_eq!(from_str.path(), &expected);
    assert_eq!(from_string.path(), &expected);
    assert_eq!(from_string_ref.path(), &expected);
    assert_eq!(from_path.path(), &expected);
    assert_eq!(from_pathbuf.path(), &expected);
    assert_eq!(from_pathbuf_ref.path(), &expected);
}

#[test]
fn test_try_new_absolute_paths() {
    use std::env;

    // Absolute paths should be used as-is, same as new()
    let temp_path = env::temp_dir().join("test_file.txt");
    let app_path = AppPath::try_new(&temp_path).unwrap();
    assert_eq!(app_path.path(), &temp_path);
    assert!(app_path.path().is_absolute());
}

#[test]
fn test_try_exe_dir_success() {
    // try_exe_dir should return the same result as exe_dir()
    let try_result = try_exe_dir().unwrap();
    let normal_result = exe_dir();
    assert_eq!(try_result, normal_result);
}

#[test]
fn test_try_exe_dir_caching() {
    // Multiple calls should return the same cached result
    let first_call = try_exe_dir().unwrap();
    let second_call = try_exe_dir().unwrap();
    let third_call = try_exe_dir().unwrap();

    assert_eq!(first_call, second_call);
    assert_eq!(second_call, third_call);

    // Should also match exe_dir()
    assert_eq!(first_call, exe_dir());
}

#[test]
fn test_mixed_api_usage() {
    // Using try_new and new together should work seamlessly
    let config1 = AppPath::try_new("config.toml").unwrap();
    let config2 = AppPath::new("config.toml");
    assert_eq!(config1, config2);

    // Using try_exe_dir and exe_dir together should work seamlessly
    let dir1 = try_exe_dir().unwrap();
    let dir2 = exe_dir();
    assert_eq!(dir1, dir2);
}

#[test]
fn test_try_new_after_exe_dir_success() {
    // If exe_dir() succeeds first, try_new() should never fail
    let _exe_dir = exe_dir(); // This might panic on first call but shouldn't in our test environment

    // Now try_new should never fail since exe_dir is cached
    let config = AppPath::try_new("config.toml").unwrap();
    let expected = exe_dir().join("config.toml");
    assert_eq!(config.path(), &expected);
}

#[test]
fn test_exe_dir_after_try_exe_dir_success() {
    // If try_exe_dir() succeeds first, exe_dir() should never panic
    let try_result = try_exe_dir().unwrap();

    // Now exe_dir should never panic since the result is cached
    let normal_result = exe_dir();
    assert_eq!(try_result, normal_result);
}

#[test]
fn test_error_type_display() {
    use std::fmt::Write;

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

// === Override API Tests ===

#[test]
fn test_new_with_override_with_override() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("config.toml");
    let result = AppPath::new_with_override("config.toml", Some(override_path.clone()));
    assert_eq!(result.path(), override_path);
}

#[test]
fn test_new_with_override_without_override() {
    let result = AppPath::new_with_override("config.toml", None::<&str>);
    let expected = exe_dir().join("config.toml");
    assert_eq!(result.path(), expected);
}

#[test]
fn test_new_with_override_fn_with_override() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("config.toml");
    let override_str = override_path.to_string_lossy().to_string();
    let result = AppPath::new_with_override_fn("config.toml", || Some(override_str));
    assert_eq!(result.path(), override_path);
}

#[test]
fn test_new_with_override_fn_without_override() {
    let result = AppPath::new_with_override_fn("config.toml", || None::<&str>);
    let expected = exe_dir().join("config.toml");
    assert_eq!(result.path(), expected);
}

#[test]
fn test_new_with_override_fn_complex_logic() {
    use std::env;

    // Test with environment variable override
    let result = AppPath::new_with_override_fn("config.toml", || {
        env::var("TEST_CONFIG_PATH")
            .ok()
            .or_else(|| env::var("FALLBACK_CONFIG").ok())
            .or_else(|| Some("fallback.toml".to_string()))
    });

    // Should use fallback since env vars don't exist
    let expected = exe_dir().join("fallback.toml");
    assert_eq!(result.path(), expected);
}

#[test]
fn test_try_new_with_override_success() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("config.toml");
    let result = AppPath::try_new_with_override("config.toml", Some(override_path.clone()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().path(), override_path);
}

#[test]
fn test_try_new_with_override_no_override() {
    let result = AppPath::try_new_with_override("config.toml", None::<&str>);
    assert!(result.is_ok());
    let expected = exe_dir().join("config.toml");
    assert_eq!(result.unwrap().path(), expected);
}

#[test]
fn test_try_new_with_override_fn_success() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("config.toml");
    let override_str = override_path.to_string_lossy().to_string();
    let result = AppPath::try_new_with_override_fn("config.toml", || Some(override_str));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().path(), override_path);
}

#[test]
fn test_try_new_with_override_fn_no_override() {
    let result = AppPath::try_new_with_override_fn("config.toml", || None::<&str>);
    assert!(result.is_ok());
    let expected = exe_dir().join("config.toml");
    assert_eq!(result.unwrap().path(), expected);
}

#[test]
fn test_override_methods_with_different_types() {
    let temp_dir = env::temp_dir();
    let string_path = temp_dir.join("string.txt").to_string_lossy().to_string();
    let pathbuf_path = temp_dir.join("pathbuf.txt");

    // Test with String
    let result1 = AppPath::new_with_override("default.txt", Some(string_path.clone()));
    assert_eq!(result1.path(), Path::new(&string_path));

    // Test with PathBuf
    let result2 = AppPath::new_with_override("default.txt", Some(pathbuf_path.clone()));
    assert_eq!(result2.path(), pathbuf_path);

    // Test with &str
    let result3 = AppPath::new_with_override("default.txt", Some(string_path.as_str()));
    assert_eq!(result3.path(), Path::new(&string_path));
}

#[test]
fn test_override_priority_order() {
    use std::env;

    // Test priority: override_fn result > default
    let result = AppPath::new_with_override_fn("default.txt", || {
        env::var("NONEXISTENT_VAR")
            .ok()
            .or_else(|| Some("priority.txt".to_string()))
    });

    let expected = exe_dir().join("priority.txt");
    assert_eq!(result.path(), expected);
}

#[test]
fn test_override_absolute_vs_relative() {
    let temp_dir = env::temp_dir();
    let absolute_override = temp_dir.join("absolute.txt");
    let relative_override = "relative.txt";

    // Absolute override should be used as-is
    let result1 = AppPath::new_with_override("default.txt", Some(absolute_override.clone()));
    assert_eq!(result1.path(), absolute_override);

    // Relative override should be resolved relative to exe_dir
    let result2 = AppPath::new_with_override("default.txt", Some(relative_override));
    let expected = exe_dir().join(relative_override);
    assert_eq!(result2.path(), expected);
}
