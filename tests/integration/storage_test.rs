//! Integration tests for storage layer

use std::path::PathBuf;

// Note: These tests require the fixtures directory to be present
// Run with: cargo test --test storage_test

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    #[test]
    fn test_load_workspace() {
        // This test verifies the fixture workspace can be loaded
        // Actual implementation would use the storage module
        let path = fixtures_path();
        assert!(path.join(".vibe-manager").exists());
    }

    #[test]
    fn test_fixture_engineers_exist() {
        let path = fixtures_path();
        assert!(path.join("alex-chen/_profile.md").exists());
        assert!(path.join("jordan-lee/_profile.md").exists());
    }

    #[test]
    fn test_fixture_meetings_exist() {
        let path = fixtures_path();
        assert!(path.join("alex-chen/2026-01-08.md").exists());
        assert!(path.join("alex-chen/2026-01-15.md").exists());
        assert!(path.join("jordan-lee/2026-01-10.md").exists());
    }
}
