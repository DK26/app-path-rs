use crate::AppPath;
use std::path::Path;

// === Basic Constructor Tests (AppPath::new) ===

#[test]
fn test_new_constructor() {
    let app_base = AppPath::new();

    // Should return an absolute path pointing to executable directory
    assert!(app_base.is_absolute());

    // Should match what std::env::current_exe() tells us (independent verification)
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    assert_eq!(&*app_base, exe_parent);

    // Should be a directory, not a file
    assert!(app_base.is_dir());

    // Should be consistent across multiple calls (caching)
    let app_base2 = AppPath::new();
    assert_eq!(app_base, app_base2);
}

#[test]
fn test_try_new_constructor() {
    let result = AppPath::try_new();
    assert!(result.is_ok());

    let app_base = result.unwrap();

    // Should return an absolute path pointing to executable directory
    assert!(app_base.is_absolute());

    // Should match what std::env::current_exe() tells us (independent verification)
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    assert_eq!(&*app_base, exe_parent);

    // Should be consistent with panicking version
    let panicking_version = AppPath::new();
    assert_eq!(app_base, panicking_version);

    // Should be a directory, not a file
    assert!(app_base.is_dir());
}

// === Path Constructor Tests (AppPath::with) ===

#[test]
fn test_new_with_different_types() {
    use std::path::PathBuf;

    // Test various input types with new
    let from_str = AppPath::with("test.txt");
    let test_string = "test.txt".to_string(); // Intentionally create String to test type
    let from_string = AppPath::with(test_string);
    let from_path_buf = AppPath::from(PathBuf::from("test.txt"));
    let from_path_ref = AppPath::from(Path::new("test.txt"));

    // All should produce equivalent results
    assert_eq!(&from_str, &from_string);
    assert_eq!(&from_string, &from_path_buf);
    assert_eq!(&from_path_buf, &from_path_ref);

    // Should all resolve to exe_dir + filename (independent verification)
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("test.txt");
    assert_eq!(&*from_str, expected);
}

#[test]
fn test_ownership_transfer() {
    use std::path::PathBuf;

    let path_buf = PathBuf::from("test.txt");
    let app_path = AppPath::with(path_buf);
    // path_buf is moved and no longer accessible

    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("test.txt");
    assert_eq!(&*app_path, expected);

    // Test with String too
    let string_path = "another_test.txt".to_string();
    let app_path2 = AppPath::with(string_path);
    // string_path is moved and no longer accessible

    let expected2 = exe_parent.join("another_test.txt");
    assert_eq!(&*app_path2, expected2);
}

#[test]
fn test_from_implementations() {
    use std::path::{Path, PathBuf};

    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("test.txt");

    // Test all From implementations
    let from_str: AppPath = "test.txt".into();
    let from_string: AppPath = "test.txt".to_string().into();
    let from_string_ref: AppPath = (&"test.txt".to_string()).into();
    let from_path: AppPath = Path::new("test.txt").into();
    let from_pathbuf: AppPath = PathBuf::from("test.txt").into();
    let from_pathbuf_ref: AppPath = (&PathBuf::from("test.txt")).into();

    // All should produce the same result
    assert_eq!(&*from_str, &expected);
    assert_eq!(&*from_string, &expected);
    assert_eq!(&*from_string_ref, &expected);
    assert_eq!(&*from_path, &expected);
    assert_eq!(&*from_pathbuf, &expected);
    assert_eq!(&*from_pathbuf_ref, &expected);
}

#[test]
fn test_from_str() {
    let rel_path = AppPath::from("config.toml");
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("config.toml");

    assert_eq!(&*rel_path, &expected);
}

#[test]
fn test_from_string() {
    let path_string = "data/file.txt".to_string();
    let rel_path = AppPath::from(path_string);
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("data/file.txt");

    assert_eq!(&*rel_path, &expected);
}

#[test]
fn test_from_string_ref() {
    let path_string = "logs/app.log".to_string();
    let rel_path = AppPath::from(&path_string);
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("logs/app.log");

    assert_eq!(&*rel_path, &expected);
}

// === Fallible API Tests ===

#[test]
fn test_try_with_success() {
    // try_with should work for all the same inputs as with()
    let config = AppPath::try_with("config.toml").unwrap();
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("config.toml");
    assert_eq!(&*config, &expected);

    let data = AppPath::try_with("data/users.db").unwrap();
    let expected = exe_parent.join("data/users.db");
    assert_eq!(&*data, &expected);
}

#[test]
fn test_try_with_different_types() {
    use std::path::{Path, PathBuf};

    // Test all the same types that work with with()
    let from_str = AppPath::try_with("config.toml").unwrap();
    let from_string = AppPath::try_with("config.toml").unwrap();
    let from_string_ref = AppPath::try_with("config.toml").unwrap();
    let from_path = AppPath::try_with(Path::new("config.toml")).unwrap();
    let from_pathbuf = AppPath::try_with(PathBuf::from("config.toml")).unwrap();
    let from_pathbuf_ref = AppPath::try_with(PathBuf::from("config.toml")).unwrap();

    // All should resolve to the same path
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("config.toml");
    assert_eq!(&*from_str, &expected);
    assert_eq!(&*from_string, &expected);
    assert_eq!(&*from_string_ref, &expected);
    assert_eq!(&*from_path, &expected);
    assert_eq!(&*from_pathbuf, &expected);
    assert_eq!(&*from_pathbuf_ref, &expected);
}

// === Override Constructor Tests ===

#[test]
fn test_with_override_some() {
    use std::env;

    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_config.toml");

    let config = AppPath::with_override("default.toml", Some(&custom_path));
    assert_eq!(&*config, &custom_path);
}

#[test]
fn test_with_override_none() {
    let config = AppPath::with_override("default.toml", None::<&str>);

    // Should fall back to default path relative to exe
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("default.toml");
    assert_eq!(&*config, &expected);
}

#[test]
fn test_try_with_override_some() {
    use std::env;

    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_config.toml");

    let config = AppPath::try_with_override("default.toml", Some(&custom_path)).unwrap();
    assert_eq!(&*config, &custom_path);
}

#[test]
fn test_try_with_override_none() {
    let config = AppPath::try_with_override("default.toml", None::<&str>).unwrap();

    // Should fall back to default path relative to exe
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("default.toml");
    assert_eq!(&*config, &expected);
}

#[test]
fn test_with_override_fn_some() {
    use std::env;

    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_fn.toml");

    let config = AppPath::with_override_fn("default.toml", || Some(custom_path.clone()));
    assert_eq!(&*config, &custom_path);
}

#[test]
fn test_with_override_fn_none() {
    let config = AppPath::with_override_fn("default.toml", || None::<String>);

    // Should fall back to default path relative to exe
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("default.toml");
    assert_eq!(&*config, &expected);
}

#[test]
fn test_try_with_override_fn_some() {
    use std::env;

    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_fn.toml");

    let config =
        AppPath::try_with_override_fn("default.toml", || Some(custom_path.clone())).unwrap();
    assert_eq!(&*config, &custom_path);
}

#[test]
fn test_try_with_override_fn_none() {
    let config = AppPath::try_with_override_fn("default.toml", || None::<String>).unwrap();

    // Should fall back to default path relative to exe
    let current_exe = std::env::current_exe().unwrap();
    let exe_parent = current_exe.parent().unwrap();
    let expected = exe_parent.join("default.toml");
    assert_eq!(&*config, &expected);
}

// === API Consistency Tests ===

#[test]
fn test_mixed_api_usage() {
    // Using try_with and with together should work seamlessly
    let config1 = AppPath::try_with("config.toml").unwrap();
    let config2 = AppPath::with("config.toml");
    assert_eq!(config1, config2);

    // Using try_new and new together should work seamlessly
    let dir1 = AppPath::try_new().unwrap();
    let dir2 = AppPath::new();
    assert_eq!(dir1, dir2);
}

#[test]
fn test_caching_consistency() {
    // Multiple calls should be consistent (tests caching)
    let first_call = AppPath::try_new().unwrap();
    let second_call = AppPath::try_new().unwrap();
    let third_call = AppPath::new();

    assert_eq!(&*first_call, &*second_call);
    assert_eq!(&*second_call, &*third_call);
}
