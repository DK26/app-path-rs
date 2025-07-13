use crate::{exe_dir, try_exe_dir, AppPath};
use std::path::Path;

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
