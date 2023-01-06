use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub commission: Decimal,
}

pub const STATE: Item<State> = Item::new("state");
