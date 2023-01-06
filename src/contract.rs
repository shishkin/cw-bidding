use cosmwasm_std::{Decimal, DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::{
    msg::InstantiateMsg,
    state::{State, STATE},
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMISSION_PERCENT: u64 = 10;
pub const DENOMINATION: &str = "ATOM";

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = match msg.owner {
        Some(owner) => deps.api.addr_validate(owner.as_str())?,
        None => info.sender,
    };

    STATE.save(
        deps.storage,
        &State {
            owner,
            commission: Decimal::percent(COMMISSION_PERCENT),
        },
    )?;

    Ok(Response::new())
}

pub mod execute {
    use cosmwasm_std::{
        BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
    };

    use crate::state::{BIDS, STATE};

    use super::DENOMINATION;

    pub fn bid(deps: DepsMut, _env: Env, info: MessageInfo) -> StdResult<Response> {
        let state = STATE.load(deps.storage)?;

        let funds = info
            .funds
            .iter()
            .find(|f| f.denom == DENOMINATION && !f.amount.is_zero())
            .unwrap();

        let fee = (funds.amount * state.commission).max(Uint128::one());
        let bid_increment = funds.amount - fee;

        BIDS.update(deps.storage, info.sender.clone(), |bid| {
            Ok::<_, StdError>(bid.unwrap_or(Uint128::zero()) + bid_increment)
        })?;

        let msg = BankMsg::Send {
            to_address: state.owner.to_string(),
            amount: vec![Coin {
                amount: fee,
                denom: DENOMINATION.to_string(),
            }],
        };

        STATE.save(deps.storage, &state)?;

        let res = Response::new()
            .add_message(msg)
            .add_attribute("action", "bid")
            .add_attribute("sender", info.sender.as_str());

        Ok(res)
    }
}

pub mod query {
    use cosmwasm_std::{Addr, Deps, StdResult};

    use crate::{msg::TotalBidResponse, state::BIDS};

    pub fn total_bid(deps: Deps, addr: Addr) -> StdResult<TotalBidResponse> {
        let bid = BIDS.may_load(deps.storage, addr)?;
        Ok(TotalBidResponse { amount: bid })
    }
}
