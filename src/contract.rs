use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::{
    msg::InstantiateMsg,
    state::{State, STATE},
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = match msg.owner {
        Some(owner) => deps.api.addr_validate(owner.as_str())?,
        None => info.sender,
    };

    STATE.save(deps.storage, &State { owner })?;

    Ok(Response::new())
}
