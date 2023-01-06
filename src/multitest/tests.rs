use cosmwasm_std::{coins, Addr};
use cw_multi_test::App;

use crate::{contract::DENOMINATION, state::STATE};

use super::BiddingContract;

#[test]
fn instantiate() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");
    let owner = Addr::unchecked("owner");
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &sender, "Bidding contract", &owner)
            .unwrap();

    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();
    assert_eq!(state.owner, owner);
}

#[test]
fn bid() {
    let owner = Addr::unchecked("owner");
    let bidder = Addr::unchecked("bidder");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &bidder, coins(10, DENOMINATION))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None).unwrap();

    contract
        .bid(&mut app, &bidder, &coins(10, DENOMINATION))
        .unwrap();

    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(1, DENOMINATION)
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(9, DENOMINATION)
    );
    assert_eq!(app.wrap().query_all_balances(bidder).unwrap(), vec![]);
}
