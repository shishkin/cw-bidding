use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::state::AddrAmount;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalBidResponse)]
    TotalBid { addr: String },

    #[returns(HighestBidResponse)]
    HighestBid {},

    #[returns(WinnerResponse)]
    Winner {},
}

#[cw_serde]
pub struct TotalBidResponse {
    pub amount: Option<Uint128>,
}

#[cw_serde]
pub struct HighestBidResponse {
    pub bid: Option<AddrAmount>,
}

#[cw_serde]
pub struct WinnerResponse {
    pub winner: Option<Addr>,
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
    Close {},
    Retract { receiver: Option<String> },
}
