use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalBidResponse)]
    TotalBid { addr: String },
}

#[cw_serde]
pub struct TotalBidResponse {
    pub amount: Option<Uint128>,
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
}
