use crate::{app_path, exe_dir, try_app_path, AppPath};
use std::env;
use std::path::PathBuf;

#[test]
fn test_app_path_macro_basic() {
    let config = app_path!("config.toml");
    let expected = AppPath::new("config.toml");
    assert_eq!(config.path(), expected.path());
}

#[test]
fn test_app_path_macro_with_env() {
    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_config.toml");
    env::set_var("TEST_CONFIG_PATH", &custom_path);

    let config = app_path!("default.toml", env = "TEST_CONFIG_PATH");
    assert_eq!(config.path(), custom_path);

    // Test with non-existent env var
    let default_config = app_path!("default.toml", env = "NON_EXISTENT_VAR");
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);

    env::remove_var("TEST_CONFIG_PATH");
}

#[test]
fn test_app_path_macro_with_override() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("custom_path.toml");
    let config = app_path!("default.toml", override = Some(override_path.clone()));
    assert_eq!(config.path(), override_path);

    let no_override: Option<PathBuf> = None;
    let default_config = app_path!("default.toml", override = no_override);
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);
}

#[test]
fn test_app_path_macro_with_fn() {
    let custom_path = env::temp_dir().join("custom_fn.toml");

    // Test fn variant that returns Some
    let config = app_path!("default.toml", fn = || Some(custom_path.clone()));
    assert_eq!(config.path(), custom_path);

    // Test fn variant that returns None
    let default_config = app_path!("default.toml", fn = || None::<PathBuf>);
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);

    // Test fn variant with complex logic
    let complex_config = app_path!("config.toml", fn = || {
        if env::var("USE_CUSTOM_CONFIG").is_ok() {
            Some(env::temp_dir().join("custom.toml"))
        } else {
            None
        }
    });
    let expected_complex = exe_dir().join("config.toml");
    assert_eq!(complex_config.path(), expected_complex);
}

// === try_app_path! Macro Tests ===

#[test]
fn test_try_app_path_macro_basic() {
    let config = try_app_path!("config.toml").unwrap();
    let expected = AppPath::try_new("config.toml").unwrap();
    assert_eq!(config.path(), expected.path());
}

#[test]
fn test_try_app_path_macro_with_env() {
    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("custom_config.toml");
    env::set_var("TEST_TRY_CONFIG_PATH", &custom_path);

    let config = try_app_path!("default.toml", env = "TEST_TRY_CONFIG_PATH").unwrap();
    assert_eq!(config.path(), custom_path);

    // Test with non-existent env var
    let default_config = try_app_path!("default.toml", env = "NON_EXISTENT_VAR").unwrap();
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);

    env::remove_var("TEST_TRY_CONFIG_PATH");
}

#[test]
fn test_try_app_path_macro_with_override() {
    let temp_dir = env::temp_dir();
    let override_path = temp_dir.join("custom_path.toml");
    let config = try_app_path!("default.toml", override = Some(override_path.clone())).unwrap();
    assert_eq!(config.path(), override_path);

    let no_override: Option<PathBuf> = None;
    let default_config = try_app_path!("default.toml", override = no_override).unwrap();
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);
}

#[test]
fn test_try_app_path_macro_with_fn() {
    let custom_path = env::temp_dir().join("custom_fn.toml");

    // Test fn variant that returns Some
    let config = try_app_path!("default.toml", fn = || Some(custom_path.clone())).unwrap();
    assert_eq!(config.path(), custom_path);

    // Test fn variant that returns None
    let default_config = try_app_path!("default.toml", fn = || None::<PathBuf>).unwrap();
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);

    // Test fn variant with complex logic
    let complex_config = try_app_path!("config.toml", fn = || {
        if env::var("USE_CUSTOM_TRY_CONFIG").is_ok() {
            Some(env::temp_dir().join("custom_try.toml"))
        } else {
            None
        }
    })
    .unwrap();
    let expected_complex = exe_dir().join("config.toml");
    assert_eq!(complex_config.path(), expected_complex);
}

#[test]
fn test_try_app_path_macro_returns_result() {
    // Test that the macro returns a Result type
    let result = try_app_path!("test.toml");
    assert!(result.is_ok());

    // Test error handling pattern
    match try_app_path!("test.toml") {
        Ok(path) => {
            assert!(path.path().ends_with("test.toml"));
        }
        Err(_) => panic!("Should not fail in normal conditions"),
    }
}

#[test]
fn test_try_app_path_vs_app_path_equivalence() {
    // The successful results should be equivalent
    let panicking = app_path!("config.toml");
    let fallible = try_app_path!("config.toml").unwrap();
    assert_eq!(panicking.path(), fallible.path());

    // Test with env override
    env::set_var("TEST_EQUIV_PATH", "/tmp/test.conf");
    let panicking_env = app_path!("default.conf", env = "TEST_EQUIV_PATH");
    let fallible_env = try_app_path!("default.conf", env = "TEST_EQUIV_PATH").unwrap();
    assert_eq!(panicking_env.path(), fallible_env.path());
    env::remove_var("TEST_EQUIV_PATH");

    // Test with custom override
    let override_path = Some(PathBuf::from("/custom/path.conf"));
    let panicking_override = app_path!("default.conf", override = override_path.clone());
    let fallible_override = try_app_path!("default.conf", override = override_path).unwrap();
    assert_eq!(panicking_override.path(), fallible_override.path());
}

#[test]
fn test_macro_fn_variants_equivalence() {
    // Test that both macros produce equivalent results when function returns None
    let panicking_fn = app_path!("test.toml", fn = || None::<String>);
    let fallible_fn = try_app_path!("test.toml", fn = || None::<String>).unwrap();
    assert_eq!(panicking_fn.path(), fallible_fn.path());

    // Test with function that returns Some
    let custom_path = env::temp_dir().join("equiv_test.toml");
    let panicking_fn_some = app_path!("test.toml", fn = || Some(custom_path.clone()));
    let fallible_fn_some = try_app_path!("test.toml", fn = || Some(custom_path.clone())).unwrap();
    assert_eq!(panicking_fn_some.path(), fallible_fn_some.path());
}

#[test]
fn test_fn_variant_with_real_xdg_logic() {
    // Test realistic XDG-style function that returns complete app config path
    fn get_config_path() -> Option<PathBuf> {
        env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|h| format!("{h}/.config")))
            .ok()
            .map(|config_dir| PathBuf::from(config_dir).join("myapp"))
    }

    // Test both macros with realistic function
    let config_app_path = app_path!("config.toml", fn = get_config_path);
    let config_try_app_path = try_app_path!("config.toml", fn = get_config_path).unwrap();

    // Both should use the same path, whether it's XDG or default
    assert_eq!(config_app_path.path(), config_try_app_path.path());

    // Check if XDG logic would be used
    if env::var("XDG_CONFIG_HOME").is_ok() || env::var("HOME").is_ok() {
        // If XDG variables are available, should use the XDG path
        let xdg_result = get_config_path();
        if let Some(xdg_path) = xdg_result {
            assert_eq!(config_app_path.path(), xdg_path);
        }
    } else {
        // If no XDG variables, should use default path
        let expected = exe_dir().join("config.toml");
        assert_eq!(config_app_path.path(), expected);
    }
}
