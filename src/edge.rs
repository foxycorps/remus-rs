use crate::ProtocolError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct EdgeFunction {
    pub id: String,
    pub name: String,
    pub version: String,
    pub runtime: String,
    pub code: Vec<u8>,
    pub config: HashMap<String, String>,
}

impl Serialize for EdgeFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("EdgeFunction", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("runtime", &self.runtime)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("config", &self.config)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for EdgeFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            id: String,
            name: String,
            version: String,
            runtime: String,
            code: Vec<u8>,
            config: HashMap<String, String>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(EdgeFunction {
            id: helper.id,
            name: helper.name,
            version: helper.version,
            runtime: helper.runtime,
            code: helper.code,
            config: helper.config,
        })
    }
}

#[derive(Debug, Clone)]
pub struct EdgeComputeResult {
    pub function_id: String,
    pub success: bool,
    pub output: Option<Vec<u8>>,
    pub error: Option<String>,
    pub execution_time: u64,
    pub resources_used: ResourceUsage,
}

impl Serialize for EdgeComputeResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("EdgeComputeResult", 6)?;
        state.serialize_field("function_id", &self.function_id)?;
        state.serialize_field("success", &self.success)?;
        state.serialize_field("output", &self.output)?;
        state.serialize_field("error", &self.error)?;
        state.serialize_field("execution_time", &self.execution_time)?;
        state.serialize_field("resources_used", &self.resources_used)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for EdgeComputeResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            function_id: String,
            success: bool,
            output: Option<Vec<u8>>,
            error: Option<String>,
            execution_time: u64,
            resources_used: ResourceUsage,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(EdgeComputeResult {
            function_id: helper.function_id,
            success: helper.success,
            output: helper.output,
            error: helper.error,
            execution_time: helper.execution_time,
            resources_used: helper.resources_used,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub network_bytes: u64,
}

pub struct EdgeCompute {
    functions: RwLock<HashMap<String, EdgeFunction>>,
}

impl EdgeCompute {
    pub fn new() -> Self {
        Self {
            functions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register_function(&self, function: EdgeFunction) -> Result<(), ProtocolError> {
        let mut functions = self.functions.write().await;
        functions.insert(function.id.clone(), function);
        Ok(())
    }

    pub async fn execute_function(
        &self,
        function_id: &str,
        input: Vec<u8>,
    ) -> Result<EdgeComputeResult, ProtocolError> {
        let functions = self.functions.read().await;
        let function = functions
            .get(function_id)
            .ok_or_else(|| ProtocolError::InvalidFormat("Function not found".into()))?;

        let start_time = std::time::Instant::now();
        
        // Here you would implement actual function execution
        // This is a placeholder that returns empty success
        let result = EdgeComputeResult {
            function_id: function_id.to_string(),
            success: true,
            output: Some(Vec::new()),
            error: None,
            execution_time: start_time.elapsed().as_millis() as u64,
            resources_used: ResourceUsage {
                cpu_time_ms: 0,
                memory_bytes: 0,
                network_bytes: 0,
            },
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[tokio::test]
    async fn test_edge_function_lifecycle() {
        let compute = EdgeCompute::new();
        
        // Create test function
        let function = EdgeFunction {
            id: "test_func".to_string(),
            name: "Test Function".to_string(),
            version: "1.0.0".to_string(),
            runtime: "wasm".to_string(),
            code: vec![0, 1, 2, 3],
            config: HashMap::new(),
        };

        // Register function
        compute.register_function(function.clone()).await.unwrap();

        // Execute function
        let result = compute.execute_function(&function.id, vec![]).await.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_edge_function_serialization() {
        let function = EdgeFunction {
            id: "test_func".to_string(),
            name: "Test Function".to_string(),
            version: "1.0.0".to_string(),
            runtime: "wasm".to_string(),
            code: vec![0, 1, 2, 3],
            config: HashMap::new(),
        };

        let serialized = serde_json::to_string(&function).unwrap();
        let deserialized: EdgeFunction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, function.id);
        assert_eq!(deserialized.code, function.code);
    }
}

// Helper functions
impl EdgeCompute {
    pub async fn list_functions(&self) -> Vec<EdgeFunction> {
        let functions = self.functions.read().await;
        functions.values().cloned().collect()
    }

    pub async fn remove_function(&self, id: &str) -> Result<(), ProtocolError> {
        let mut functions = self.functions.write().await;
        functions.remove(id).ok_or_else(|| {
            ProtocolError::InvalidFormat("Function not found".into())
        })?;
        Ok(())
    }
}