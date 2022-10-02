use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

// registry of addresses and the amount they sent to the contract's bank account
pub const CONTRIBUTIONS: Map<&Addr, Uint128> = Map::new("contributions");

// coin threshold of rewards
pub const THRESHOLD_COIN: Item<Coin> = Item::new("threshold-coin");
// timestamp of when reward should be distributed
pub const DEADLINE: Item<Timestamp> = Item::new("deadline");
// receiver of reward
pub const RECEIVER: Item<String> = Item::new("receiver");
