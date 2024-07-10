use std::path::PathBuf;

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let id = uuid::Uuid::new_v4();
        let path = std::env::temp_dir().join(id.to_string());
        std::fs::create_dir_all(&path).expect("Failed to create temp dir");
        Self(path)
    }

    fn close(&self) {
        std::fs::remove_dir_all(&self.0).expect("Failed to remove temp dir");
    }
}

struct AssertDirContent {
    source: PathBuf,
    target: PathBuf,
}

impl AssertDirContent {
    fn new(source: PathBuf, target: PathBuf) -> Self {
        Self { source, target }
    }

    fn assert(&self) {
        let source = self.source.read_dir().expect("Failed to read source dir");
        let target = self.target.read_dir().expect("Failed to read target dir");

        for entry in source {
            let entry = entry.expect("Failed to read entry");
            let name = entry.file_name();
            let target = self.target.join(name);

            assert!(target.exists(), "File not found: {:?}", target);
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
fn test_single() {}
