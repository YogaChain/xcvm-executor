use crate::state::StateManager;
use crate::verifier::Verifier;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub output: Option<String>,
}

pub struct XCVMExecutor {
    state_manager: StateManager,
    verifier: Verifier,
}

impl XCVMExecutor {
    /// Initializes the contract executor
    pub fn new(state_manager: StateManager, verifier: Verifier) -> Self {
        Self { state_manager, verifier }
    }

    /// Executes a smart contract function
    pub fn execute_contract(&self, contract_name: &str, function: &str, params: Vec<String>) -> ExecutionResult {
        if self.verifier.verify_execution(contract_name) {
            let result = format!("Executed `{}` on `{}` with {:?}", function, contract_name, params);
            ExecutionResult {
                success: true,
                gas_used: 21000,
                output: Some(result),
            }
        } else {
            ExecutionResult {
                success: false,
                gas_used: 0,
                output: None,
            }
        }
    }
}
