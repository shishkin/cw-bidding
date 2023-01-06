use contract::{execute, query};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::{ExecMsg, InstantiateMsg, QueryMsg};

mod contract;
pub mod error;
pub mod msg;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TotalBid { addr } => {
            let addr = deps.api.addr_validate(&addr)?;
            to_binary(&query::total_bid(deps, addr)?)
        }
        QueryMsg::HighestBid {} => to_binary(&query::highest_bid(deps)?),
        QueryMsg::Winner {} => to_binary(&query::winner(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecMsg::Bid {} => execute::bid(deps, env, info),
        ExecMsg::Close {} => execute::close(deps, env, info),
        ExecMsg::Retract { receiver } => {
            let receiver = match receiver {
                None => info.sender.clone(),
                Some(r) => deps.api.addr_validate(&r)?,
            };
            execute::retract(deps, env, info, receiver)
        }
    }
}
