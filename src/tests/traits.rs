use crate::{AppPath, AppPathError};
use std::path::{Path, PathBuf};

// === AsRef<Path> Trait Tests ===

#[test]
fn test_as_ref_path() {
    let app_path = AppPath::new("config.toml");
    let path_ref: &Path = app_path.as_ref();
    assert!(path_ref.ends_with("config.toml"));

    // Should be able to use in functions expecting AsRef<Path>
    fn takes_path_ref<P: AsRef<Path>>(path: P) -> String {
        path.as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    let filename = takes_path_ref(&app_path);
    assert_eq!(filename, "config.toml");
}

#[test]
fn test_as_ref_path_with_nested() {
    let nested_path = AppPath::new("config/deep/app.toml");
    let path_ref: &Path = nested_path.as_ref();
    assert!(
        path_ref.ends_with("config/deep/app.toml") || path_ref.ends_with("config\\deep\\app.toml")
    );
}

// === Into<PathBuf> Trait Tests ===

#[test]
fn test_into_pathbuf() {
    let app_path = AppPath::new("config.toml");
    let path_buf: PathBuf = app_path.into();
    assert!(path_buf.ends_with("config.toml"));
}

#[test]
fn test_into_pathbuf_complex() {
    let complex_path = AppPath::new("data/config/settings.json");
    let path_buf: PathBuf = complex_path.into();
    assert!(
        path_buf.ends_with("data/config/settings.json")
            || path_buf.ends_with("data\\config\\settings.json")
    );
    assert_eq!(path_buf.file_name().unwrap(), "settings.json");
}

// === Display Trait Tests ===

#[test]
fn test_display_trait() {
    let app_path = AppPath::new("config.toml");
    let displayed = format!("{app_path}");
    assert!(displayed.ends_with("config.toml"));

    // Should be readable and contain the path
    assert!(displayed.contains("config.toml"));
}

#[test]
fn test_display_with_complex_path() {
    let complex_path = AppPath::new("data/nested/config/app.json");
    let displayed = format!("{complex_path}");
    assert!(displayed.contains("app.json"));
    assert!(displayed.contains("config"));
    assert!(displayed.contains("nested"));
}

// === Debug Trait Tests ===

#[test]
fn test_debug_trait() {
    let app_path = AppPath::new("config.toml");
    let debug_str = format!("{app_path:?}");

    // Debug output should contain useful information
    assert!(debug_str.contains("config.toml"));
    // Debug typically shows more internal structure
    assert!(debug_str.len() > "config.toml".len());
}

#[test]
fn test_debug_trait_detailed() {
    let app_path = AppPath::new("test.toml");
    let debug_output = format!("{app_path:#?}");

    // Pretty debug should be well-formatted
    assert!(debug_output.contains("test.toml"));
}

// === Clone Trait Tests ===

#[test]
fn test_clone_trait() {
    let original = AppPath::new("config.toml");
    let cloned = original.clone();

    assert_eq!(original.path(), cloned.path());
    assert!(cloned.path().ends_with("config.toml"));
}

#[test]
fn test_clone_independence() {
    let original = AppPath::new("original.toml");
    let cloned = original.clone();

    // Changes to the path should not affect the clone
    // (though AppPath is immutable, so this is more of a conceptual test)
    assert_eq!(original.file_name(), cloned.file_name());
    assert_eq!(original.parent(), cloned.parent());
}

// === PartialEq and Eq Traits Tests ===

#[test]
fn test_partial_eq_same_path() {
    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("config.toml");
    assert_eq!(path1, path2);
}

#[test]
fn test_partial_eq_different_paths() {
    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("settings.toml");
    assert_ne!(path1, path2);
}

#[test]
fn test_partial_eq_with_normalization() {
    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("./config.toml");
    // These might be equal after normalization, depending on implementation
    // The exact behavior depends on how the library handles path normalization
    let _ = path1 == path2; // Just verify it compiles and doesn't panic
}

// === Hash Trait Tests ===

#[test]
fn test_hash_trait() {
    use std::collections::HashMap;

    let path1 = AppPath::new("config.toml");
    let path2 = AppPath::new("config.toml");
    let path3 = AppPath::new("settings.toml");

    let mut map = HashMap::new();
    map.insert(path1.clone(), "config data");
    map.insert(path3, "settings data");

    // Should be able to look up the same path
    assert_eq!(map.get(&path2), Some(&"config data"));
}

#[test]
fn test_hash_consistency() {
    use std::collections::HashSet;

    let paths = vec![
        AppPath::new("config.toml"),
        AppPath::new("settings.toml"),
        AppPath::new("data.json"),
        AppPath::new("config.toml"), // Duplicate
    ];

    let unique_paths: HashSet<_> = paths.into_iter().collect();
    assert_eq!(unique_paths.len(), 3); // Should deduplicate the config.toml
}

// === PartialOrd and Ord Traits Tests ===

#[test]
fn test_partial_ord() {
    let path1 = AppPath::new("a.toml");
    let path2 = AppPath::new("b.toml");
    let path3 = AppPath::new("c.toml");

    assert!(path1 < path2);
    assert!(path2 < path3);
    assert!(path1 < path3);
}

#[test]
fn test_ord_sorting() {
    let mut paths = [
        AppPath::new("z.toml"),
        AppPath::new("a.toml"),
        AppPath::new("m.toml"),
    ];

    paths.sort();

    assert!(paths[0].path().ends_with("a.toml"));
    assert!(paths[1].path().ends_with("m.toml"));
    assert!(paths[2].path().ends_with("z.toml"));
}

// === Send and Sync Traits Tests ===

#[test]
fn test_send_trait() {
    fn assert_send<T: Send>() {}
    assert_send::<AppPath>();

    // Should be able to send across threads
    let path = AppPath::new("config.toml");
    let handle = std::thread::spawn(move || format!("{path}"));

    let result = handle.join().unwrap();
    assert!(result.contains("config.toml"));
}

#[test]
fn test_sync_trait() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<AppPath>();

    // Should be able to share across threads
    use std::sync::Arc;

    let path = Arc::new(AppPath::new("shared.toml"));
    let path_clone = Arc::clone(&path);

    let handle = std::thread::spawn(move || {
        let name = path_clone.file_name();
        name.map(|n| n.to_owned())
    });

    let result = handle.join().unwrap();
    assert_eq!(result, path.file_name().map(|n| n.to_owned()));
}

// === Custom Trait Implementations Tests ===

#[test]
fn test_from_pathbuf() {
    // Test if there's a From<PathBuf> implementation
    let path_buf = PathBuf::from("test.toml");
    let app_path = AppPath::new(path_buf);
    assert!(app_path.path().ends_with("test.toml"));
}

#[test]
fn test_from_str() {
    // Test string-like construction
    let app_path = AppPath::new("config.toml");
    assert!(app_path.path().ends_with("config.toml"));

    let app_path_from_string = AppPath::new(String::from("settings.toml"));
    assert!(app_path_from_string.path().ends_with("settings.toml"));
}

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

// === Deref Trait Tests ===

#[test]
fn test_deref_to_path() {
    let app_path = AppPath::new("config.toml");

    // Should be able to call Path methods directly
    assert!(app_path.ends_with("config.toml"));
    assert_eq!(app_path.file_name().unwrap(), "config.toml");
    assert_eq!(app_path.extension().unwrap(), "toml");
}

#[test]
fn test_deref_path_methods() {
    let nested_path = AppPath::new("config/deep/app.toml");

    // All Path methods should be available
    assert!(nested_path.is_absolute());
    assert!(!nested_path.is_relative());
    assert_eq!(nested_path.file_stem().unwrap(), "app");

    let parent = nested_path.parent().unwrap();
    assert!(parent.ends_with("deep"));
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
