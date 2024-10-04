use alloy_primitives::B256;
use alloy_sol_types::sol;
use consensus_core::types::Forks;
use consensus_core::types::{FinalityUpdate, LightClientStore, Update};
use serde::{Deserialize, Serialize};
use ssz_rs::prelude::*;
pub use ssz_rs::prelude::{Bitvector, Vector};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProofInputs {
    pub updates: Vec<Update>,
    pub finality_update: FinalityUpdate,
    pub expected_current_slot: u64,
    pub store: LightClientStore,
    pub genesis_root: B256,
    pub forks: Forks,
    pub execution_state_proof: ExecutionStateProof,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExecutionStateProof {
    #[serde(rename = "executionStateRoot")]
    pub execution_state_root: B256,
    #[serde(rename = "executionStateBranch")]
    pub execution_state_branch: Vec<B256>,
    pub gindex: String,
}

pub type ProofOutputs = sol! {
    tuple(bytes32, bytes32, bytes32, bytes32, uint256, uint256, bytes32)
};