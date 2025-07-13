use crate::{app_path, exe_dir, try_app_path};
use std::env;
use std::path::PathBuf;

// === Environment Variable Override Tests ===

#[test]
fn test_env_override_with_string() {
    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("env_override.toml");
    env::set_var("TEST_ENV_OVERRIDE", &custom_path);

    let config = app_path!("default.toml", env = "TEST_ENV_OVERRIDE");
    assert_eq!(config.path(), custom_path);

    env::remove_var("TEST_ENV_OVERRIDE");
}

#[test]
fn test_env_override_with_nonexistent_var() {
    let config = app_path!("default.toml", env = "DEFINITELY_NONEXISTENT_VAR");
    let expected = exe_dir().join("default.toml");
    assert_eq!(config.path(), expected);
}

#[test]
fn test_env_override_empty_value() {
    env::set_var("EMPTY_ENV_VAR", "");

    let config = app_path!("default.toml", env = "EMPTY_ENV_VAR");
    // Empty env var creates AppPath with empty string, which resolves to the
    // directory where the test binary is executed from (target/debug/deps/)
    let expected_path = config.path().to_path_buf();
    assert!(expected_path.to_string_lossy().contains("target"));

    // Verify it's a directory path (ends with separator)
    assert!(
        config.path().is_dir()
            || config
                .path()
                .to_string_lossy()
                .ends_with(std::path::MAIN_SEPARATOR)
    );

    env::remove_var("EMPTY_ENV_VAR");
}

#[test]
fn test_env_override_relative_path() {
    env::set_var("RELATIVE_PATH_VAR", "config/test.toml");

    let config = app_path!("default.toml", env = "RELATIVE_PATH_VAR");
    // Relative path from env var is relative to current dir, not exe dir
    let expected = exe_dir().join("config/test.toml");
    assert_eq!(config.path(), expected);

    env::remove_var("RELATIVE_PATH_VAR");
}

#[test]
fn test_env_override_absolute_path() {
    let temp_dir = env::temp_dir();
    let absolute_path = temp_dir.join("absolute_test.toml");
    env::set_var("ABSOLUTE_PATH_VAR", &absolute_path);

    let config = app_path!("default.toml", env = "ABSOLUTE_PATH_VAR");
    assert_eq!(config.path(), absolute_path);

    env::remove_var("ABSOLUTE_PATH_VAR");
}

// === Direct Override Tests ===

#[test]
fn test_direct_override_with_some_pathbuf() {
    let override_path = if cfg!(windows) {
        PathBuf::from("C:\\custom\\override\\path.toml")
    } else {
        PathBuf::from("/custom/override/path.toml")
    };
    let config = app_path!("default.toml", override = Some(override_path.clone()));
    assert_eq!(config.path(), override_path);
}

#[test]
fn test_direct_override_with_some_string() {
    let override_path = if cfg!(windows) {
        Some("C:\\custom\\override\\string.toml")
    } else {
        Some("/custom/override/string.toml")
    };
    let config = app_path!("default.toml", override = override_path);
    let expected = if cfg!(windows) {
        PathBuf::from("C:\\custom\\override\\string.toml")
    } else {
        PathBuf::from("/custom/override/string.toml")
    };
    assert_eq!(config.path(), expected);
}

#[test]
fn test_direct_override_with_none() {
    let override_path: Option<PathBuf> = None;
    let config = app_path!("default.toml", override = override_path);
    let expected = exe_dir().join("default.toml");
    assert_eq!(config.path(), expected);
}

#[test]
fn test_direct_override_with_variable() {
    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("variable_override.toml");
    let maybe_override = Some(custom_path.clone());

    let config = app_path!("default.toml", override = maybe_override);
    assert_eq!(config.path(), custom_path);

    let no_override: Option<PathBuf> = None;
    let default_config = app_path!("default.toml", override = no_override);
    let expected = exe_dir().join("default.toml");
    assert_eq!(default_config.path(), expected);
}

// === Function Override Tests ===

#[test]
fn test_fn_override_returning_some() {
    let custom_path = env::temp_dir().join("fn_override.toml");

    let config = app_path!("default.toml", fn = || Some(custom_path.clone()));
    assert_eq!(config.path(), custom_path);
}

#[test]
fn test_fn_override_returning_none() {
    let config = app_path!("default.toml", fn = || None::<PathBuf>);
    let expected = exe_dir().join("default.toml");
    assert_eq!(config.path(), expected);
}

#[test]
fn test_fn_override_with_conditional_logic() {
    // Test function that checks environment conditions
    let config = app_path!("config.toml", fn = || {
        if env::var("USE_TEMP_CONFIG").is_ok() {
            Some(env::temp_dir().join("temp_config.toml"))
        } else {
            None
        }
    });

    // Without env var, should use default
    let expected = exe_dir().join("config.toml");
    assert_eq!(config.path(), expected);

    // With env var, should use temp dir
    env::set_var("USE_TEMP_CONFIG", "1");
    let config_with_env = app_path!("config.toml", fn = || {
        if env::var("USE_TEMP_CONFIG").is_ok() {
            Some(env::temp_dir().join("temp_config.toml"))
        } else {
            None
        }
    });
    let expected_temp = env::temp_dir().join("temp_config.toml");
    assert_eq!(config_with_env.path(), expected_temp);

    env::remove_var("USE_TEMP_CONFIG");
}

#[test]
fn test_fn_override_with_xdg_style_logic() {
    fn get_xdg_config_path() -> Option<PathBuf> {
        env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|h| format!("{h}/.config")))
            .ok()
            .map(|base| PathBuf::from(base).join("myapp").join("config.toml"))
    }

    let config = app_path!("config.toml", fn = get_xdg_config_path);

    // The exact result depends on the environment, but we can verify it's callable
    assert!(config.path().ends_with("config.toml"));
}

#[test]
fn test_fn_override_with_platform_specific_logic() {
    fn get_platform_config_path() -> Option<PathBuf> {
        if cfg!(windows) {
            env::var("APPDATA")
                .ok()
                .map(|appdata| PathBuf::from(appdata).join("MyApp").join("config.toml"))
        } else {
            env::var("HOME").ok().map(|home| {
                PathBuf::from(home)
                    .join(".config")
                    .join("myapp")
                    .join("config.toml")
            })
        }
    }

    let config = app_path!("config.toml", fn = get_platform_config_path);
    assert!(config.path().ends_with("config.toml"));
}

// === Combined Override Tests ===

#[test]
fn test_env_override_fallback_to_default() {
    // When env var doesn't exist, should fall back to default path
    let config = app_path!("fallback.toml", env = "NONEXISTENT_ENV_VAR");
    let expected = exe_dir().join("fallback.toml");
    assert_eq!(config.path(), expected);
}

#[test]
fn test_multiple_override_scenarios() {
    let temp_dir = env::temp_dir();

    // Test 1: env override works
    let env_path = temp_dir.join("env_config.toml");
    env::set_var("MULTI_TEST_ENV", &env_path);
    let config1 = app_path!("default.toml", env = "MULTI_TEST_ENV");
    assert_eq!(config1.path(), env_path);

    // Test 2: direct override works
    let direct_path = temp_dir.join("direct_config.toml");
    let config2 = app_path!("default.toml", override = Some(direct_path.clone()));
    assert_eq!(config2.path(), direct_path);

    // Test 3: function override works
    let fn_path = temp_dir.join("fn_config.toml");
    let config3 = app_path!("default.toml", fn = || Some(fn_path.clone()));
    assert_eq!(config3.path(), fn_path);

    // Test 4: no override falls back to default
    let config4 = app_path!("default.toml");
    let expected = exe_dir().join("default.toml");
    assert_eq!(config4.path(), expected);

    env::remove_var("MULTI_TEST_ENV");
}

// === try_app_path! Override Tests ===

#[test]
fn test_try_app_path_env_override() {
    let temp_dir = env::temp_dir();
    let custom_path = temp_dir.join("try_env_override.toml");
    env::set_var("TRY_TEST_ENV_OVERRIDE", &custom_path);

    let config = try_app_path!("default.toml", env = "TRY_TEST_ENV_OVERRIDE").unwrap();
    assert_eq!(config.path(), custom_path);

    env::remove_var("TRY_TEST_ENV_OVERRIDE");
}

#[test]
fn test_try_app_path_direct_override() {
    let override_path = if cfg!(windows) {
        PathBuf::from("C:\\custom\\try\\override.toml")
    } else {
        PathBuf::from("/custom/try/override.toml")
    };
    let config = try_app_path!("default.toml", override = Some(override_path.clone())).unwrap();
    assert_eq!(config.path(), override_path);
}

#[test]
fn test_try_app_path_fn_override() {
    let custom_path = env::temp_dir().join("try_fn_override.toml");

    let config = try_app_path!("default.toml", fn = || Some(custom_path.clone())).unwrap();
    assert_eq!(config.path(), custom_path);
}

#[test]
fn test_try_app_path_override_equivalence() {
    // Verify try_app_path! and app_path! produce the same results for overrides
    let temp_dir = env::temp_dir();
    let test_path = temp_dir.join("equivalence_test.toml");

    // Test direct override equivalence
    let panicking = app_path!("test.toml", override = Some(test_path.clone()));
    let fallible = try_app_path!("test.toml", override = Some(test_path.clone())).unwrap();
    assert_eq!(panicking.path(), fallible.path());

    // Test env override equivalence
    env::set_var("EQUIV_TEST_ENV", &test_path);
    let panicking_env = app_path!("test.toml", env = "EQUIV_TEST_ENV");
    let fallible_env = try_app_path!("test.toml", env = "EQUIV_TEST_ENV").unwrap();
    assert_eq!(panicking_env.path(), fallible_env.path());
    env::remove_var("EQUIV_TEST_ENV");

    // Test fn override equivalence
    let panicking_fn = app_path!("test.toml", fn = || Some(test_path.clone()));
    let fallible_fn = try_app_path!("test.toml", fn = || Some(test_path.clone())).unwrap();
    assert_eq!(panicking_fn.path(), fallible_fn.path());
}
