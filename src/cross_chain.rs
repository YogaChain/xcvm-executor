use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossChainState {
    pub contract_name: String,
    pub state_data: HashMap<String, String>,
}

pub struct CrossChainManager {
    state_registry: HashMap<String, CrossChainState>,
}

impl CrossChainManager {
    pub fn new() -> Self {
        Self {
            state_registry: HashMap::new(),
        }
    }

    /// Updates state across chains
    pub fn update_state(&mut self, contract_name: &str, key: &str, value: &str) {
        self.state_registry
            .entry(contract_name.to_string())
            .or_insert(CrossChainState {
                contract_name: contract_name.to_string(),
                state_data: HashMap::new(),
            })
            .state_data.insert(key.to_string(), value.to_string());
    }

    /// Retrieves state from a specific blockchain
    pub fn get_state(&self, contract_name: &str, key: &str) -> Option<&String> {
        self.state_registry.get(contract_name)?.state_data.get(key)
    }
}
