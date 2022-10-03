use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

/// Registry of addresses and the amount they sent to the contract's bank account.
pub const CONTRIBUTIONS: Map<&Addr, Uint128> = Map::new("contributions");

/// Coin threshold of rewards.
pub const THRESHOLD_COIN: Item<Coin> = Item::new("threshold-coin");
/// Timestamp of when reward should be distributed.
pub const DEADLINE: Item<Timestamp> = Item::new("deadline");
/// Receiver of reward.
pub const RECEIVER: Item<String> = Item::new("receiver");
