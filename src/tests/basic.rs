use crate::{exe_dir, AppPath};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Helper to create a file at a given path for testing.
pub fn create_test_file(path: &Path) {
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(b"test content").unwrap();
}

#[allow(dead_code)]
/// Helper for expected executable-relative path
pub fn expect_exe_rel(path: &str) -> PathBuf {
    exe_dir().join(path)
}

#[test]
fn resolves_relative_path_to_exe_dir() {
    let rel = "config.toml";
    let rel_path = AppPath::new(rel);
    let expected = exe_dir().join(rel);

    assert_eq!(&*rel_path, &expected);
    assert!(rel_path.is_absolute());
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
    let expected_path = temp_dir.join(rel);

    // Demonstrating the improved patterns - use as_ref() or deref coercion
    let as_ref_path: &Path = rel_path.as_ref();
    assert_eq!(as_ref_path, expected_path.as_path());
    assert_eq!(&*rel_path, expected_path.as_path());
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
