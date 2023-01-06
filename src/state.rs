use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub commission: Decimal,
}

pub const STATE: Item<State> = Item::new("state");
pub const BIDS: Map<Addr, Uint128> = Map::new("bids");
