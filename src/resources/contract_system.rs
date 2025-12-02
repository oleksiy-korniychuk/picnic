use bevy::prelude::*;
use crate::components::inventory::Inventory;

/// Resource managing all active contracts
#[derive(Resource, Debug)]
pub struct ContractSystem {
    pub active_contracts: Vec<Contract>,
}

impl Default for ContractSystem {
    fn default() -> Self {
        Self {
            active_contracts: vec![
                // POC: Single hardcoded contract
                Contract {
                    id: "contract_001".to_string(),
                    description: "Bring back one Fully Empty from the Zone".to_string(),
                    requirements: vec![
                        ItemRequirement {
                            item_name: "Fully Empty".to_string(),
                            quantity: 1,
                        },
                    ],
                    completed: false,
                },
            ],
        }
    }
}

impl ContractSystem {
    /// Check if all contracts are completed based on player inventory
    pub fn validate_contracts(&mut self, inventory: &Inventory) -> Vec<ContractStatus> {
        self.active_contracts
            .iter_mut()
            .map(|contract| {
                let all_requirements_met = contract.requirements.iter().all(|req| {
                    let count = inventory
                        .items
                        .iter()
                        .filter(|item| item.name == req.item_name)
                        .count() as u32;
                    count >= req.quantity
                });

                contract.completed = all_requirements_met;

                ContractStatus {
                    description: contract.description.clone(),
                    completed: all_requirements_met,
                }
            })
            .collect()
    }

    /// Reset all contracts to incomplete state
    pub fn reset(&mut self) {
        for contract in &mut self.active_contracts {
            contract.completed = false;
        }
    }
}

/// Represents a contract/objective for the player
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: String,
    pub description: String,
    pub requirements: Vec<ItemRequirement>,
    pub completed: bool,
}

/// Represents a required item for contract completion
#[derive(Debug, Clone)]
pub struct ItemRequirement {
    pub item_name: String,
    pub quantity: u32,
}

/// Status of a contract for UI display
#[derive(Debug, Clone)]
pub struct ContractStatus {
    pub description: String,
    pub completed: bool,
}
