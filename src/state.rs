use crate::ProtocolError;
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVersion {
    pub version: u64,
    pub timestamp: u64,
    pub checksum: [u8; 32],
}

#[derive(Debug)]
pub struct StateManager {
    state: RwLock<HashMap<String, Bytes>>,
    versions: RwLock<Vec<StateVersion>>,
    max_versions: usize,
}

impl StateManager {
    pub fn new(max_versions: usize) -> Self {
        Self {
            state: RwLock::new(HashMap::new()),
            versions: RwLock::new(Vec::with_capacity(max_versions)),
            max_versions,
        }
    }

    pub async fn apply_delta(&self, key: String, delta: Bytes) -> Result<StateVersion, ProtocolError> {
        let mut state = self.state.write().await;
        let mut versions = self.versions.write().await;

        // Apply delta and create new version
        let new_state = if let Some(current) = state.get(&key) {
            // Merge current state with delta
            let mut merged = BytesMut::from(current.as_ref());
            merged.extend_from_slice(&delta);
            merged.freeze()
        } else {
            delta
        };

        // Update state
        state.insert(key, new_state.clone());

        // Create new version
        let version = StateVersion {
            version: versions.len() as u64 + 1,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            checksum: Self::calculate_checksum(&new_state),
        };

        // Add version and maintain history limit
        versions.push(version.clone());
        if versions.len() > self.max_versions {
            versions.remove(0);
        }

        Ok(version)
    }

    pub async fn get_state(&self, key: &str) -> Option<Bytes> {
        let state = self.state.read().await;
        state.get(key).cloned()
    }

    pub async fn validate_version(&self, version: &StateVersion) -> bool {
        let versions = self.versions.read().await;
        versions.iter().any(|v| v.version == version.version && v.checksum == version.checksum)
    }

    fn calculate_checksum(data: &[u8]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[tokio::test]
    async fn test_state_manager() {
        let manager = StateManager::new(5);
        
        // Test state update
        let key = "test_key".to_string();
        let delta = Bytes::from("initial state");
        let version = manager.apply_delta(key.clone(), delta).await.unwrap();
        
        // Verify state
        let state = manager.get_state(&key).await.unwrap();
        assert_eq!(state, Bytes::from("initial state"));
        
        // Verify version
        assert!(manager.validate_version(&version).await);
    }

    #[tokio::test]
    async fn test_version_limit() {
        let manager = StateManager::new(2);
        let key = "test_key".to_string();

        // Create 3 versions
        for i in 0..3 {
            let delta = Bytes::from(format!("state {}", i));
            manager.apply_delta(key.clone(), delta).await.unwrap();
        }

        // Verify only 2 versions are kept
        let versions = manager.versions.read().await;
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, 2);
        assert_eq!(versions[1].version, 3);
    }
}

// Helper functions
impl StateManager {
    pub async fn get_version_history(&self) -> Vec<StateVersion> {
        self.versions.read().await.clone()
    }

    pub async fn clear_state(&self) {
        let mut state = self.state.write().await;
        let mut versions = self.versions.write().await;
        state.clear();
        versions.clear();
    }
} 