use crate::{app_path, exe_dir, AppPath};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

// === Path Component Tests ===

#[test]
fn test_file_name() {
    let path = app_path!("config.toml");
    assert_eq!(path.file_name(), Some(OsStr::new("config.toml")));

    let path_with_dir = app_path!("config/app.toml");
    assert_eq!(path_with_dir.file_name(), Some(OsStr::new("app.toml")));

    let dir_path = app_path!("config/");
    assert_eq!(dir_path.file_name(), Some(OsStr::new("config")));
}

#[test]
fn test_file_stem() {
    let path = app_path!("config.toml");
    assert_eq!(path.file_stem(), Some(OsStr::new("config")));

    let complex_name = app_path!("app.config.toml");
    assert_eq!(complex_name.file_stem(), Some(OsStr::new("app.config")));

    let no_extension = app_path!("README");
    assert_eq!(no_extension.file_stem(), Some(OsStr::new("README")));
}

#[test]
fn test_extension() {
    let toml_file = app_path!("config.toml");
    assert_eq!(toml_file.extension(), Some(OsStr::new("toml")));

    let json_file = app_path!("data.json");
    assert_eq!(json_file.extension(), Some(OsStr::new("json")));

    let no_extension = app_path!("README");
    assert_eq!(no_extension.extension(), None);

    let multiple_dots = app_path!("archive.tar.gz");
    assert_eq!(multiple_dots.extension(), Some(OsStr::new("gz")));
}

#[test]
fn test_parent() {
    let nested_path = app_path!("config/app.toml");
    let parent = nested_path.parent().unwrap();
    assert!(parent.ends_with("config"));

    let root_file = app_path!("app.toml");
    let parent_of_root = root_file.parent().unwrap();
    // Parent should be the exe directory
    assert_eq!(parent_of_root.path(), exe_dir());
}

// === Path Joining and Manipulation ===

#[test]
fn test_join() {
    let base = app_path!("config");
    let joined = base.join("app.toml");
    assert!(joined.ends_with("config/app.toml") || joined.ends_with("config\\app.toml"));

    let base_file = app_path!("config.toml");
    let joined_to_file = base_file.join("nested");
    assert!(
        joined_to_file.ends_with("config.toml/nested")
            || joined_to_file.ends_with("config.toml\\nested")
    );
}

#[test]
fn test_with_file_name() {
    let original = app_path!("config.toml");
    let renamed = AppPath::new(original.with_file_name("settings.toml"));
    assert!(renamed.ends_with("settings.toml"));
    assert!(!renamed.ends_with("config.toml"));

    // Should maintain the same parent directory
    assert_eq!(original.parent(), renamed.parent());
}

#[test]
fn test_with_extension() {
    let toml_file = app_path!("config.toml");
    let json_file = toml_file.with_extension("json");
    assert!(json_file.ends_with("config.json"));
    assert!(!json_file.ends_with("config.toml"));

    let no_ext_file = app_path!("README");
    let with_ext = no_ext_file.with_extension("md");
    assert!(with_ext.ends_with("README.md"));
}

// === Path Comparison and Relationships ===

#[test]
fn test_starts_with() {
    let exe_path = exe_dir();
    let config_path = app_path!("config.toml");

    // App paths should start with the exe directory
    assert!(config_path.starts_with(exe_path));

    let nested_path = app_path!("config/app.toml");
    assert!(nested_path.starts_with(exe_path));
    assert!(nested_path.starts_with(config_path.parent().unwrap()));
}

#[test]
fn test_ends_with() {
    let config_path = app_path!("config.toml");
    assert!(config_path.ends_with("config.toml"));

    let nested_path = app_path!("data/settings/app.toml");
    assert!(nested_path.ends_with("app.toml"));
    assert!(nested_path.ends_with("settings/app.toml"));
    assert!(nested_path.ends_with("data/settings/app.toml"));
}

#[test]
fn test_strip_prefix() {
    let exe_path = exe_dir();
    let config_path = app_path!("config/app.toml");

    let relative = config_path.strip_prefix(exe_path).unwrap();
    assert_eq!(relative, Path::new("config/app.toml"));
}

// === Path Canonicalization and Absolute Paths ===

#[test]
fn test_is_absolute() {
    let app_path = app_path!("config.toml");
    assert!(app_path.is_absolute());

    let nested_path = app_path!("config/deep/nested/file.toml");
    assert!(nested_path.is_absolute());
}

#[test]
fn test_is_relative() {
    let app_path = app_path!("config.toml");
    assert!(!app_path.is_relative());

    // All app paths should be absolute
    let any_path = app_path!("any/path/structure.toml");
    assert!(!any_path.is_relative());
}

// === Component Iteration ===

#[test]
fn test_components() {
    let path = app_path!("config/nested/file.toml");
    let components: Vec<_> = path.components().collect();

    // Should have multiple components including the file name
    assert!(components.len() > 1);

    // Last component should be the file
    let last = components.last().unwrap();
    assert_eq!(last.as_os_str(), "file.toml");
}

#[test]
fn test_iter() {
    let path = app_path!("config/app.toml");
    let parts: Vec<_> = path.iter().collect();

    // Should contain at least the config directory and filename
    assert!(parts.contains(&OsStr::new("config")));
    assert!(parts.contains(&OsStr::new("app.toml")));
}

// === Path Creation and Ancestors ===

#[test]
fn test_ancestors() {
    let nested_path = app_path!("config/deep/nested/file.toml");
    let ancestors: Vec<_> = nested_path.ancestors().collect();

    // Should include the path itself and all parent directories
    assert!(ancestors.len() > 3);
    assert_eq!(ancestors[0], &*nested_path);
    assert!(ancestors[1].ends_with("nested"));
    assert!(ancestors[2].ends_with("deep"));
    assert!(ancestors[3].ends_with("config"));
}

// === String Conversion and Display ===

#[test]
fn test_to_string_lossy() {
    let path = app_path!("config.toml");
    let string_repr = path.to_string_lossy();
    assert!(string_repr.ends_with("config.toml"));
}

#[test]
fn test_to_path_buf() {
    let app_path = app_path!("config.toml");
    let path_buf: PathBuf = app_path.to_path_buf();
    assert_eq!(&*app_path, path_buf.as_path());
}

#[test]
fn test_as_os_str() {
    let path = app_path!("config.toml");
    let os_str = path.as_os_str();
    assert!(os_str.to_string_lossy().ends_with("config.toml"));
}

// === Complex Path Manipulations ===

#[test]
fn test_complex_path_building() {
    let base = app_path!("data");
    let config_dir = base.join("config");
    let settings_file = config_dir.join("settings.toml");
    let backup_file = settings_file.with_extension("backup");

    assert!(
        backup_file.ends_with("data/config/settings.backup")
            || backup_file.ends_with("data\\config\\settings.backup")
    );
    assert!(backup_file.starts_with(exe_dir()));
}

#[test]
fn test_path_normalization() {
    // Test that redundant path components are handled
    let path = app_path!("config/../config/app.toml");
    let normalized = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    // Should still be valid and end with the expected file
    assert!(normalized.ends_with("config/app.toml") || normalized.ends_with("config\\app.toml"));
}

#[test]
fn test_path_with_special_characters() {
    let special_path = app_path!("config with spaces.toml");
    assert!(special_path.ends_with("config with spaces.toml"));
    assert_eq!(
        special_path.file_name(),
        Some(OsStr::new("config with spaces.toml"))
    );

    let unicode_path = app_path!("configürâtion.toml");
    assert!(unicode_path.ends_with("configürâtion.toml"));
    assert_eq!(unicode_path.file_stem(), Some(OsStr::new("configürâtion")));
}

// === Platform-Specific Path Tests ===

#[cfg(windows)]
#[test]
fn test_windows_path_separators() {
    let path = app_path!("config\\app.toml");
    assert!(path.ends_with("config\\app.toml") || path.ends_with("config/app.toml"));

    // Test that forward slashes are normalized on Windows
    let forward_slash_path = app_path!("config/app.toml");
    let backslash_path = app_path!("config\\app.toml");

    // Both should reference the same logical path
    assert_eq!(forward_slash_path.file_name(), backslash_path.file_name());
}

#[cfg(unix)]
#[test]
fn test_unix_path_separators() {
    let path = app_path!("config/app.toml");
    assert!(path.ends_with("config/app.toml"));
    assert_eq!(path.file_name(), Some(OsStr::new("app.toml")));
}

// === Edge Cases ===

#[test]
fn test_root_file_manipulation() {
    let root_file = app_path!("app.toml");

    // Should be able to get parent (exe directory)
    let parent = root_file.parent().unwrap();
    assert_eq!(parent.path(), exe_dir());

    // Should be able to change extension
    let json_version = root_file.with_extension("json");
    assert!(json_version.ends_with("app.json"));

    // Should be able to rename
    let renamed = AppPath::new(root_file.with_file_name("settings.toml"));
    assert!(renamed.ends_with("settings.toml"));
    assert_eq!(renamed.parent(), root_file.parent());
}

#[test]
fn test_empty_path_components() {
    // Test paths with empty components
    let path_with_double_slash = app_path!("config//app.toml");
    assert!(path_with_double_slash.ends_with("app.toml"));

    let path_with_dot = app_path!("config/./app.toml");
    assert!(path_with_dot.ends_with("app.toml"));
}

#[test]
fn test_path_comparison() {
    let path1 = app_path!("config.toml");
    let path2 = app_path!("config.toml");
    let path3 = app_path!("settings.toml");

    assert_eq!(&*path1, &*path2);
    assert_ne!(&*path1, &*path3);

    // Test lexicographic ordering
    assert!(*path1 < *path3); // "config" < "settings"
}

// === into_inner() Method Tests ===

#[test]
fn test_into_inner_basic() {
    let app_path = app_path!("config.toml");
    let expected_path = app_path.to_path_buf();

    let inner_path: PathBuf = app_path.into_inner();

    assert_eq!(inner_path, expected_path);
    assert!(inner_path.is_absolute());
    assert!(inner_path.ends_with("config.toml"));
}

#[test]
fn test_into_inner_with_nested_path() {
    let app_path = app_path!("config/settings/app.toml");
    let expected_path = app_path.to_path_buf();

    let inner_path: PathBuf = app_path.into_inner();

    assert_eq!(inner_path, expected_path);
    assert!(inner_path.is_absolute());
    assert!(inner_path.ends_with("config/settings/app.toml"));
}

#[test]
fn test_into_inner_with_directory_path() {
    let app_path = app_path!("data/cache/");
    let expected_path = app_path.to_path_buf();

    let inner_path: PathBuf = app_path.into_inner();

    assert_eq!(inner_path, expected_path);
    assert!(inner_path.is_absolute());
    assert!(inner_path.ends_with("data/cache"));
}

#[test]
fn test_into_inner_type_consistency() {
    let app_path = app_path!("test.txt");

    // Verify the returned type is exactly PathBuf
    let inner: PathBuf = app_path.into_inner();

    // Should be able to use all PathBuf methods
    let _display = inner.display();
    let _components: Vec<_> = inner.components().collect();
    let _extension = inner.extension();
    let _file_name = inner.file_name();

    // Should be convertible to standard path types
    let _path_ref: &Path = inner.as_path();
    let _os_str = inner.as_os_str();
}

#[test]
fn test_into_inner_ownership_transfer() {
    let app_path = app_path!("owned.txt");
    let original_path = app_path.to_path_buf();

    // Move ownership with into_inner
    let inner_path = app_path.into_inner();

    // Verify the path is the same
    assert_eq!(inner_path, original_path);

    // app_path is now consumed and cannot be used
    // This test verifies that we truly get ownership of the inner PathBuf
    drop(inner_path); // Explicit drop to show ownership
}

#[test]
fn test_into_inner_with_special_characters() {
    let app_path = app_path!("files with spaces/üñíçøðé.txt");
    let expected_path = app_path.to_path_buf();

    let inner_path: PathBuf = app_path.into_inner();

    assert_eq!(inner_path, expected_path);
    assert!(inner_path.is_absolute());
    assert!(inner_path.to_string_lossy().contains("üñíçøðé.txt"));
}

#[test]
fn test_into_inner_with_override() {
    // Test case 1: Override with a custom path (completely replaces default)
    let custom_path = std::env::temp_dir().join("custom_config.toml");
    let app_path = AppPath::with_override("config.toml", Some(&custom_path));
    let inner_path: PathBuf = app_path.into_inner();

    // When override is Some, it completely replaces the default path
    assert_eq!(inner_path, custom_path);
    assert!(inner_path.is_absolute());
    assert!(inner_path.ends_with("custom_config.toml"));

    // Test case 2: No override, should use default relative to exe_dir
    let app_path_default = AppPath::with_override("config.toml", None::<&str>);
    let inner_path_default: PathBuf = app_path_default.into_inner();
    let expected_default = exe_dir().join("config.toml");

    assert_eq!(inner_path_default, expected_default);
    assert!(inner_path_default.ends_with("config.toml"));
}
