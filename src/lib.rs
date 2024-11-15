//!
//! Stylus Player Info Contract
//! 
//! This contract is used to store player information

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{Address, U256, U64};
use stylus_sdk::{msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU64}};

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

    //update match results (only allow the matchmaking contract to do it)
    fn add_match_results(&mut self, player_address: Address, was_won: bool) -> Result<(), Vec<u8>>{
        if self.matchmaking_contract.get() != msg::sender(){
            return Err("Only the matchmaking contract can update match results".into());
        }
        let player_info = self.player_info.get(player_address);
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        
        let player_total_matches = player_info.total_matches.get();
        let player_winning_matches = player_info.winning_matches.get();

        let mut player_info_setter = self.player_info.setter(player_address);
        player_info_setter.total_matches.set(player_total_matches + U64::from(1));

        if was_won {
            player_info_setter.winning_matches.set(player_winning_matches + U64::from(1));
        }

        Ok(())
    }

    //get player's total matches
    fn get_total_matches(&self) -> Result<U256, Vec<u8>>{
        let player_info = self.player_info.get(msg::sender());
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        //encoded u64 to u256 for displaying purposes
        Ok(U256::from(player_info.total_matches.get()))
    }

    //get an address's total matches
    fn get_total_matches_by_address(&self, player_address: Address) -> Result<U256, Vec<u8>>{
        let player_info = self.player_info.get(player_address);
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        //encoded u64 to u256 for displaying purposes
        Ok(U256::from(player_info.total_matches.get()))
    }

    //get player's winning matches
    fn get_winning_matches(&self) -> Result<U256, Vec<u8>>{
        let player_info = self.player_info.get(msg::sender());
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        //encoded u64 to u256 for displaying purposes
        Ok(U256::from(player_info.winning_matches.get()))
    }

    //get an address's winning matches
    fn get_winning_matches_by_address(&self, player_address: Address) -> Result<U256, Vec<u8>>{
        let player_info = self.player_info.get(player_address);
        if !player_info.exists.get(){
            return Err("Player does not exist".into());
        }
        //encoded u64 to u256 for displaying purposes
        Ok(U256::from(player_info.winning_matches.get()))
    }
}

