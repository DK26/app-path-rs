use crate::AppPath;

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
