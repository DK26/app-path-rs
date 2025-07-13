use crate::AppPath;
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

// === From Trait Tests ===

#[test]
fn test_from_pathbuf() {
    let original_path = PathBuf::from("config.toml");
    let app_path = AppPath::from(original_path.clone());

    // Note: From trait uses AppPath resolution, so path will be resolved relative to exe dir
    // Check that the resolved path ends with the original filename
    assert!(app_path.path().ends_with("config.toml"));
}

#[test]
fn test_from_str() {
    // AppPath doesn't implement FromStr trait, use new() instead
    let app_path = AppPath::new("config.toml");
    assert!(app_path.path().ends_with("config.toml"));
}

// === Borrow Trait Tests ===

#[test]
fn test_borrow_checker_friendly() {
    use std::borrow::Borrow;

    let app_path = AppPath::new("config.toml");
    let borrowed: &Path = app_path.borrow();
    assert!(borrowed.ends_with("config.toml"));

    // Should work with collections that use Borrow
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(app_path, "config_value");

    // Should be able to lookup using a path
    let lookup_path = AppPath::new("config.toml").to_path_buf();
    let borrowed_lookup: &Path = &lookup_path;
    // This tests that Borrow is implemented correctly for lookups
    assert!(map.contains_key(borrowed_lookup));
}

// === Collection Operations Tests ===

#[test]
fn test_collection_operations() {
    let paths = vec![
        AppPath::new("a.txt"),
        AppPath::new("b.txt"),
        AppPath::new("c.txt"),
    ];

    // Should work with iterators and standard library functions
    let path_buf_vec: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    assert_eq!(path_buf_vec.len(), 3);

    // All should end with .txt
    for path in &path_buf_vec {
        assert!(path.to_string_lossy().ends_with(".txt"));
    }
}

// === Integration with Standard Library Functions ===

#[test]
fn test_works_with_std_functions() {
    let app_path = AppPath::new("test_file.txt");

    // Should work with std::fs functions
    let _metadata_result = std::fs::metadata(&app_path); // Won't panic, just may return error

    // Should work with path manipulation
    let parent = app_path.parent();
    assert!(parent.is_some());

    // Should work with path joining
    let joined = app_path.join("subfile.txt");
    assert!(joined.path().to_string_lossy().contains("test_file.txt"));
}
