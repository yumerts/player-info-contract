# Player Info Smart Contract

> This smart contract stores player's stats to be queried and used for various purposes within other parts of the application (mainly serve as a data store than anything else)

## Key Features

```
#[storage]
pub struct PlayerInfo{
    exists: StorageBool, //checks if this player actually exists or not
    display_name: StorageString, //the display name within the game
    winning_matches: StorageU64,
    total_matches: StorageU64,
    winning_predictions: StorageU64,
    total_predictions: StorageU64
}

#[storage]
#[entrypoint]
pub struct PlayerInfoContract{
    initialized: StorageBool,
    owner: StorageAddress, //the owner who would be able to set the other two contracts
    matchmaking_contract: StorageAddress,
    prediction_contract: StorageAddress,
    player_info: StorageMap<Address, PlayerInfo>
}
```

## Interface

### Solidity ABI Interface

```
// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface IPlayerInfoContract {
    function init() external;

    function getMatchmakingContract() external view returns (address);

    function getPredictionContract() external view returns (address);

    function setMatchmakingContract(address matchmaking_contract) external;

    function setPredictionContract(address prediction_contract) external;

    function registerPlayer() external;

    function updateDisplayName(string calldata display_name) external;

    function getDisplayName() external view returns (string memory);

    function getDisplayNameByAddress(address player_address) external view returns (string memory);

    function addMatchResults(address player_address, bool was_won) external;

    function getTotalMatches() external view returns (uint256);

    function getTotalMatchesByAddress(address player_address) external view returns (uint256);

    function getWinningMatches() external view returns (uint256);

    function getWinningMatchesByAddress(address player_address) external view returns (uint256);

    function addPredictionResults(address player_address, bool was_won) external;

    function getTotalPredictions() external view returns (uint256);

    function getTotalPredictionsByAddress(address player_address) external view returns (uint256);

    function getWinningPredictions() external view returns (uint256);

    function getWinningPredictionsByAddress(address player_address) external view returns (uint256);
}
```
### Json ABI

```
[{"inputs":[{"internalType":"address","name":"player_address","type":"address"},{"internalType":"bool","name":"was_won","type":"bool"}],"name":"addMatchResults","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"},{"internalType":"bool","name":"was_won","type":"bool"}],"name":"addPredictionResults","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"getDisplayName","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"}],"name":"getDisplayNameByAddress","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getMatchmakingContract","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getPredictionContract","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getTotalMatches","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"}],"name":"getTotalMatchesByAddress","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getTotalPredictions","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"}],"name":"getTotalPredictionsByAddress","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getWinningMatches","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"}],"name":"getWinningMatchesByAddress","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getWinningPredictions","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"player_address","type":"address"}],"name":"getWinningPredictionsByAddress","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"init","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"registerPlayer","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"matchmaking_contract","type":"address"}],"name":"setMatchmakingContract","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prediction_contract","type":"address"}],"name":"setPredictionContract","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"display_name","type":"string"}],"name":"updateDisplayName","outputs":[],"stateMutability":"nonpayable","type":"function"}]
```