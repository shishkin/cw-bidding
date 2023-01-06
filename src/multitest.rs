use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{execute, instantiate, msg::InstantiateMsg, query};

#[cfg(test)]
mod tests;

pub struct BiddingContract(Addr);

impl BiddingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        owner: impl Into<Option<&'a Addr>>,
    ) -> StdResult<BiddingContract> {
        let owner = owner.into();
        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                owner: owner.map(Addr::to_string),
            },
            &[],
            label,
            None,
        )
        .map_err(|err| err.downcast().unwrap())
        .map(BiddingContract)
    }
}
