use crate::AppPath;
use std::sync::Arc;

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
    let path = Arc::new(AppPath::new("shared.toml"));
    let path_clone: Arc<AppPath> = Arc::clone(&path);

    let handle = std::thread::spawn(move || {
        let name = path_clone.file_name();
        name.map(|n| n.to_owned())
    });

    let result = handle.join().unwrap();
    assert_eq!(result, path.file_name().map(|n| n.to_owned()));
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
