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
            highest_bid: None,
            winner: None,
        },
    )?;

    Ok(Response::new())
}

pub mod execute {
    use cosmwasm_std::{Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

    use crate::{
        error::ContractError,
        state::{AddrAmount, BIDS, STATE},
    };

    use super::DENOMINATION;

    pub fn bid(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let mut state = STATE.load(deps.storage)?;

        let funds = info
            .funds
            .iter()
            .find(|f| f.denom == DENOMINATION && !f.amount.is_zero())
            .unwrap();

        let fee = (funds.amount * state.commission).max(Uint128::one());
        let bid_increment = funds.amount - fee;
        let previous_bid = BIDS.may_load(deps.storage, info.sender.clone())?;
        let current_bid = previous_bid.unwrap_or_default() + bid_increment;
        let highest_bid = state.highest_bid.map(|b| b.amount).unwrap_or_default();

        if current_bid <= highest_bid {
            return Err(ContractError::InsufficientBid { min: highest_bid });
        }

        state.highest_bid = Some(AddrAmount {
            addr: info.sender.clone(),
            amount: current_bid,
        });
        STATE.save(deps.storage, &state)?;

        BIDS.save(deps.storage, info.sender.clone(), &current_bid)?;

        let msg = BankMsg::Send {
            to_address: state.owner.to_string(),
            amount: vec![Coin {
                amount: fee,
                denom: DENOMINATION.to_string(),
            }],
        };

        let res = Response::new()
            .add_message(msg)
            .add_attribute("action", "bid")
            .add_attribute("sender", info.sender.as_str());

        Ok(res)
    }

    pub fn close(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let mut state = STATE.load(deps.storage)?;

        if state.owner != info.sender {
            return Err(ContractError::Unauthorized);
        }

        let mut res = Response::new();

        if let Some(winner) = state.highest_bid.clone() {
            state.winner = Some(winner.addr.clone());
            STATE.save(deps.storage, &state)?;

            let msg = BankMsg::Send {
                to_address: state.owner.to_string(),
                amount: vec![Coin {
                    amount: winner.amount,
                    denom: DENOMINATION.to_string(),
                }],
            };
            res = res
                .add_message(msg)
                .add_attribute("winning_bid", winner.amount);

            BIDS.remove(deps.storage, winner.addr.clone());
        }

        res = res
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender.as_str());

        Ok(res)
    }

    pub fn retract(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        receiver: Addr,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        let mut res = Response::new();

        if let Some(winner) = state.winner {
            if winner == info.sender {
                return Err(ContractError::WinnerCannotRetract);
            }

            if let Some(amount) = BIDS.may_load(deps.storage, info.sender.clone())? {
                if !amount.is_zero() {
                    let msg = BankMsg::Send {
                        to_address: receiver.to_string(),
                        amount: vec![Coin {
                            amount,
                            denom: DENOMINATION.to_string(),
                        }],
                    };
                    res = res
                        .add_message(msg)
                        .add_attribute("retracted_amount", amount)
                        .add_attribute("retracted_to", receiver);
                    BIDS.remove(deps.storage, info.sender.clone());
                }
            }
        } else {
            return Err(ContractError::BidsStillOpen);
        }

        res = res
            .add_attribute("action", "retract")
            .add_attribute("sender", info.sender.as_str());

        Ok(res)
    }
}

pub mod query {
    use cosmwasm_std::{Addr, Deps, StdResult};

    use crate::{
        msg::{HighestBidResponse, TotalBidResponse, WinnerResponse},
        state::{BIDS, STATE},
    };

    pub fn total_bid(deps: Deps, addr: Addr) -> StdResult<TotalBidResponse> {
        let bid = BIDS.may_load(deps.storage, addr)?;
        Ok(TotalBidResponse { amount: bid })
    }

    pub fn highest_bid(deps: Deps) -> StdResult<HighestBidResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(HighestBidResponse {
            bid: state.highest_bid,
        })
    }

    pub fn winner(deps: Deps) -> StdResult<WinnerResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(WinnerResponse {
            winner: state.winner,
        })
    }
}
