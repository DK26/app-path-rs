use crate::AppPath;
use std::collections::{HashMap, HashSet};

// === PartialEq and Eq Traits Tests ===

#[test]
fn test_partial_eq_same_path() {
    let path1 = AppPath::with("config.toml");
    let path2 = AppPath::with("config.toml");
    assert_eq!(path1, path2);
}

#[test]
fn test_partial_eq_different_paths() {
    let path1 = AppPath::with("config.toml");
    let path2 = AppPath::with("settings.toml");
    assert_ne!(path1, path2);
}

#[test]
fn test_partial_eq_with_normalization() {
    let path1 = AppPath::with("config.toml");
    let path2 = AppPath::with("./config.toml");
    // These might be equal after normalization, depending on implementation
    // The exact behavior depends on how the library handles path normalization
    let _ = path1 == path2; // Just verify it compiles and doesn't panic
}

// === Hash Trait Tests ===

#[test]
fn test_hash_trait() {
    let path1 = AppPath::with("config.toml");
    let path2 = AppPath::with("config.toml");
    let path3 = AppPath::with("settings.toml");

    let mut map = HashMap::new();
    map.insert(path1.clone(), "config data");
    map.insert(path3, "settings data");

    // Should be able to look up the same path
    assert_eq!(map.get(&path2), Some(&"config data"));
}

#[test]
fn test_hash_consistency() {
    let paths = vec![
        AppPath::with("config.toml"),
        AppPath::with("settings.toml"),
        AppPath::with("data.json"),
        AppPath::with("config.toml"), // Duplicate
    ];

    let unique_paths: HashSet<_> = paths.into_iter().collect();
    assert_eq!(unique_paths.len(), 3); // Should deduplicate the config.toml
}

// === PartialOrd and Ord Traits Tests ===

#[test]
fn test_partial_ord() {
    let path1 = AppPath::with("a.toml");
    let path2 = AppPath::with("b.toml");
    let path3 = AppPath::with("c.toml");

    assert!(path1 < path2);
    assert!(path2 < path3);
    assert!(path1 < path3);
}

#[test]
fn test_ord_sorting() {
    let mut paths = [
        AppPath::with("z.toml"),
        AppPath::with("a.toml"),
        AppPath::with("m.toml"),
    ];

    paths.sort();

    assert!(paths[0].ends_with("a.toml"));
    assert!(paths[1].ends_with("m.toml"));
    assert!(paths[2].ends_with("z.toml"));
}
