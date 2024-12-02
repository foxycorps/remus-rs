use crate::ProtocolError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Represents the health status of a service
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Information about a registered service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub address: SocketAddr,
    pub metadata: HashMap<String, String>,
    pub last_seen: SystemTime,
    pub health_status: HealthStatus,
}

/// Registry for service discovery and health monitoring
pub struct ServiceRegistry {
    services: RwLock<HashMap<String, ServiceInfo>>,
    ttl: Duration,
}

impl ServiceRegistry {
    /// Creates a new registry with the specified TTL for services
    pub fn new(ttl: Duration) -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            ttl,
        }
    }

    /// Registers a service in the registry
    pub async fn register(&self, info: ServiceInfo) {
        let mut services = self.services.write().await;
        services.insert(info.id.clone(), info);
    }

    /// Removes a service from the registry
    pub async fn unregister(&self, id: &str) {
        let mut services = self.services.write().await;
        services.remove(id);
    }

    /// Retrieves information about a specific service
    pub async fn get_service(&self, id: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(id).cloned()
    }

    /// Queries services based on a filter function
    pub async fn query(&self, filter: impl Fn(&ServiceInfo) -> bool) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services
            .values()
            .filter(|s| filter(s))
            .cloned()
            .collect()
    }

    /// Removes expired services based on TTL
    pub async fn cleanup_expired(&self) {
        let now = SystemTime::now();
        let mut services = self.services.write().await;
        services.retain(|_, info| {
            info.last_seen
                .elapsed()
                .map(|elapsed| elapsed < self.ttl)
                .unwrap_or(false)
        });
    }

    /// Helper function to update service health status
    pub async fn update_health(&self, id: &str, status: HealthStatus) -> Result<(), ProtocolError> {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(id) {
            service.health_status = status;
            service.last_seen = SystemTime::now();
            Ok(())
        } else {
            Err(ProtocolError::InvalidFormat("Service not found".into()))
        }
    }

    /// Helper function to get all healthy services
    pub async fn get_healthy_services(&self) -> Vec<ServiceInfo> {
        self.query(|s| s.health_status == HealthStatus::Healthy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn create_test_service(id: &str) -> ServiceInfo {
        ServiceInfo {
            id: id.to_string(),
            name: "test_service".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["test".to_string()],
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            metadata: HashMap::new(),
            last_seen: SystemTime::now(),
            health_status: HealthStatus::Healthy,
        }
    }

    #[tokio::test]
    async fn test_service_registration() {
        let registry = ServiceRegistry::new(Duration::from_secs(60));
        let service = create_test_service("test1");
        
        registry.register(service.clone()).await;
        let retrieved = registry.get_service("test1").await.unwrap();
        
        assert_eq!(retrieved.id, service.id);
        assert_eq!(retrieved.health_status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_service_expiration() {
        let registry = ServiceRegistry::new(Duration::from_nanos(1));
        let service = create_test_service("test2");
        
        registry.register(service).await;
        tokio::time::sleep(Duration::from_millis(1)).await;
        
        registry.cleanup_expired().await;
        assert!(registry.get_service("test2").await.is_none());
    }

    #[tokio::test]
    async fn test_service_health_update() {
        let registry = ServiceRegistry::new(Duration::from_secs(60));
        let service = create_test_service("test3");
        
        registry.register(service).await;
        registry.update_health("test3", HealthStatus::Degraded).await.unwrap();
        
        let updated = registry.get_service("test3").await.unwrap();
        assert_eq!(updated.health_status, HealthStatus::Degraded);
    }

    #[tokio::test]
    async fn test_service_query() {
        let registry = ServiceRegistry::new(Duration::from_secs(60));
        
        registry.register(create_test_service("test4")).await;
        registry.register(create_test_service("test5")).await;
        
        let services = registry.query(|s| s.version == "1.0.0").await;
        assert_eq!(services.len(), 2);
    }
} 