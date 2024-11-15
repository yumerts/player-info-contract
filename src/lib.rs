//!
//! Stylus Player Info Contract
//! 
//! This contract is used to store player information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU64}};

#[storage]
pub struct PlayerInfo{
    exists: StorageBool,
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

    // Initializes the contract
    fn init(&mut self) -> Result<(), Vec<u8>>{
        let initialized = self.initialized.get();
        if initialized{
            return Err("Contract has already been initialized".into());
        }
        self.owner.set(msg::sender());
        Ok(())
    }

    // reads contract addresses
    fn get_matchmaking_contract(&self) -> Address{
        self.matchmaking_contract.get()
    }

    fn get_prediction_contract(&self) -> Address{
        self.prediction_contract.get()
    }

    // allows the owner of the contract to set specific functions
    fn set_matchmaking_contract(&mut self, matchmaking_contract: Address) -> Result<(), Vec<u8>>{
        if self.owner.get() != msg::sender(){
            return Err("Only the owner can set the matchmaking contract".into());
        }
        self.matchmaking_contract.set(matchmaking_contract);
        Ok(())
    }

    fn set_prediction_contract(&mut self, prediction_contract: Address) -> Result<(), Vec<u8>>{
        if self.owner.get() != msg::sender(){
            return Err("Only the owner can set the prediction contract".into());
        }
        self.prediction_contract.set(prediction_contract);
        Ok(())
    }

    // allows any addresses to register as a player
    fn register_player(&mut self) -> Result<(), Vec<u8>>{
        let mut player_info_setter =  self.player_info.setter(msg::sender());
        player_info_setter.exists.set(true);
        player_info_setter.display_name.set_str(msg::sender().to_string());
        Ok(())
    }
    
    // allows the player to update their display name
    fn update_display_name(&mut self, display_name: String) -> Result<(), Vec<u8>>{
        let player_info = self.player_info.get(msg::sender());
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        
        let mut player_info_setter = self.player_info.setter(msg::sender());
        player_info_setter.display_name.set_str(&display_name);
        Ok(())
    }

    //get the sender's display name
    fn get_display_name(&self) -> Result<String, Vec<u8>>{
        let player_info = self.player_info.get(msg::sender());
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        Ok(player_info.display_name.get_string())
    }

    //get some wallet's display name
    fn get_display_name_by_address(&self, player_address: Address) -> Result<String, Vec<u8>>{
        let player_info = self.player_info.get(player_address);
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        Ok(player_info.display_name.get_string())
    }

}

