use std::env::current_exe;
use std::path::{Path, PathBuf};

/// Creates paths relative to the executable location for applications.
/// All files and directories stay together with the executable.
#[derive(Clone, Debug)]
pub struct AppPath {
    relative_path: PathBuf,
    full_path: PathBuf,
}

impl AppPath {
    /// Creates file paths relative to the executable location.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let exe_dir = current_exe()?
            .parent()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not determine executable parent directory",
                )
            })?
            .to_path_buf();

        Ok(Self {
            relative_path: path.as_ref().to_path_buf(),
            full_path: exe_dir.join(path),
        })
    }

    /// Override the base directory (useful for testing or custom layouts)
    pub fn with_base(mut self, base: impl AsRef<Path>) -> Self {
        self.full_path = base.as_ref().join(&self.relative_path);
        self
    }

    /// Get the relative portion of the path
    pub fn relative(&self) -> &Path {
        &self.relative_path
    }

    /// Get the full resolved path
    pub fn full(&self) -> &Path {
        &self.full_path
    }

    /// Check if the path exists
    pub fn exists(&self) -> bool {
        self.full_path.exists()
    }

    /// Create parent directories if they don't exist
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        if let Some(parent) = self.full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for AppPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path.display())
    }
}

impl From<AppPath> for PathBuf {
    fn from(relative_path: AppPath) -> Self {
        relative_path.full_path
    }
}
impl AsRef<Path> for AppPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.full_path.as_ref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    /// Helper to create a file at a given path for testing.
    fn create_test_file(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(path).unwrap();
        writeln!(file, "test").unwrap();
    }

    #[test]
    fn resolves_relative_path_to_exe_dir() {
        // Simulate a file relative to the executable
        let rel = "myconfig.toml";
        let rel_path = AppPath::new(rel).unwrap();
        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join(rel);

        assert_eq!(rel_path.full_path, expected);
        assert!(rel_path.full_path.is_absolute());
    }

    #[test]
    fn resolves_relative_path_with_custom_base() {
        // Use a temp directory as the base
        let temp_dir = env::temp_dir().join("app_path_test_base");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "subdir/file.txt";
        let rel_path = AppPath::new(rel).unwrap().with_base(&temp_dir);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.full_path, expected);
        assert!(rel_path.full_path.is_absolute());
    }

    #[test]
    fn can_access_file_using_full_path() {
        // Actually create a file and check that the full path points to it
        let temp_dir = env::temp_dir().join("app_path_test_access");
        let file_name = "access.txt";
        let file_path = temp_dir.join(file_name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        create_test_file(&file_path);

        let rel_path = AppPath::new(file_name).unwrap().with_base(&temp_dir);
        assert!(rel_path.exists());
        assert_eq!(rel_path.full_path, file_path);
    }

    #[test]
    fn handles_dot_and_dotdot_components() {
        let temp_dir = env::temp_dir().join("app_path_test_dot");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "./foo/../bar.txt";
        let rel_path = AppPath::new(rel).unwrap().with_base(&temp_dir);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.full_path, expected);
    }

    #[test]
    fn as_ref_and_into_pathbuf_are_consistent() {
        let rel = "somefile.txt";
        let rel_path = AppPath::new(rel).unwrap();
        let as_ref_path: &Path = rel_path.as_ref();
        let into_pathbuf: PathBuf = rel_path.clone().into();
        assert_eq!(as_ref_path, into_pathbuf.as_path());
    }

    #[test]
    fn test_relative_method() {
        let rel = "config/app.toml";
        let rel_path = AppPath::new(rel).unwrap();
        assert_eq!(rel_path.relative(), Path::new(rel));
    }

    #[test]
    fn test_full_method() {
        let rel = "data/file.txt";
        let temp_dir = env::temp_dir().join("app_path_test_full");
        let rel_path = AppPath::new(rel).unwrap().with_base(&temp_dir);
        assert_eq!(rel_path.full(), temp_dir.join(rel));
    }

    #[test]
    fn test_exists_method() {
        let temp_dir = env::temp_dir().join("app_path_test_exists");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let file_name = "exists_test.txt";
        let file_path = temp_dir.join(file_name);
        create_test_file(&file_path);

        let rel_path = AppPath::new(file_name).unwrap().with_base(&temp_dir);
        assert!(rel_path.exists());

        let non_existent = AppPath::new("non_existent.txt")
            .unwrap()
            .with_base(&temp_dir);
        assert!(!non_existent.exists());
    }

    #[test]
    fn test_create_dir_all() {
        let temp_dir = env::temp_dir().join("app_path_test_create");
        let _ = fs::remove_dir_all(&temp_dir);

        let rel = "deep/nested/dir/file.txt";
        let rel_path = AppPath::new(rel).unwrap().with_base(&temp_dir);

        rel_path.create_dir_all().unwrap();
        assert!(rel_path.full_path.parent().unwrap().exists());
    }

    #[test]
    fn test_display_trait() {
        let rel = "display_test.txt";
        let temp_dir = env::temp_dir().join("app_path_test_display");
        let rel_path = AppPath::new(rel).unwrap().with_base(&temp_dir);

        let expected = temp_dir.join(rel);
        assert_eq!(format!("{}", rel_path), format!("{}", expected.display()));
    }
}
