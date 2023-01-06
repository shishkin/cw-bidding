use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{
    error::ContractError,
    execute, instantiate,
    msg::{
        ExecMsg, HighestBidResponse, InstantiateMsg, QueryMsg, TotalBidResponse, WinnerResponse,
    },
    query,
};

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

    #[track_caller]
    pub fn bid<'a>(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.addr().clone(), &ExecMsg::Bid {}, funds)
            .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn close<'a>(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.addr().clone(),
            &ExecMsg::Close {},
            &vec![],
        )
        .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn retract<'a>(
        &self,
        app: &mut App,
        sender: &Addr,
        receiver: impl Into<Option<Addr>>,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.addr().clone(),
            &ExecMsg::Retract {
                receiver: receiver.into().map(|r| r.to_string()),
            },
            &vec![],
        )
        .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }
    pub fn query_total_bid(&self, app: &App, addr: &Addr) -> StdResult<TotalBidResponse> {
        app.wrap().query_wasm_smart(
            self.addr(),
            &QueryMsg::TotalBid {
                addr: addr.to_string(),
            },
        )
    }

    pub fn query_highest_bid(&self, app: &App) -> StdResult<HighestBidResponse> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::HighestBid {})
    }

    pub fn query_winner(&self, app: &App) -> StdResult<WinnerResponse> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Winner {})
    }
}
