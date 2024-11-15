//!
//! Stylus Player Info Contract
//! 
//! This contract is used to store player information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU64}};

#[storage]
pub struct PlayerInfo{
    display_name: StorageString,
    winning_matches: StorageU64,
    total_matches: StorageU64,
    winning_predictions: StorageU64,
    total_predictions: StorageU64
}

#[storage]
#[entrypoint]
pub struct PlayerInfoContract{
    initialized: StorageBool,
    owner: StorageAddress,
    matchmaking_contract: StorageAddress,
    prediction_contract: StorageAddress,
    player_info: StorageMap<Address, PlayerInfo>
}

#[public]
impl PlayerInfoContract{

}

