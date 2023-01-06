use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct AddrAmount {
    pub addr: Addr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub commission: Decimal,
    pub highest_bid: Option<AddrAmount>,
}

pub const STATE: Item<State> = Item::new("state");
pub const BIDS: Map<Addr, Uint128> = Map::new("bids");
