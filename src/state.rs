use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractState {
    pub contract_name: String,
    pub state_data: HashMap<String, String>,
}

pub struct StateManager {
    contracts: HashMap<String, ContractState>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    /// Updates a contract's internal state
    pub fn update_state(&mut self, contract_name: &str, key: &str, value: &str) {
        self.contracts
            .entry(contract_name.to_string())
            .or_insert(ContractState {
                contract_name: contract_name.to_string(),
                state_data: HashMap::new(),
            })
            .state_data.insert(key.to_string(), value.to_string());
    }

    /// Retrieves a contract's stored state
    pub fn get_state(&self, contract_name: &str, key: &str) -> Option<&String> {
        self.contracts.get(contract_name)?.state_data.get(key)
    }
}
