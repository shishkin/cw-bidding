use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::App;

use crate::{
    contract::DENOMINATION,
    error::ContractError,
    state::{AddrAmount, STATE},
};

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
            .init_balance(storage, &bidder, coins(100, DENOMINATION))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None).unwrap();

    contract
        .bid(&mut app, &bidder, &coins(40, DENOMINATION))
        .unwrap();
    contract
        .bid(&mut app, &bidder, &coins(60, DENOMINATION))
        .unwrap();

    let resp = contract.query_total_bid(&app, &bidder).unwrap();
    assert_eq!(resp.amount, Some(Uint128::new(90)));

    let resp = contract.query_total_bid(&app, &owner).unwrap();
    assert_eq!(resp.amount, None);

    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(10, DENOMINATION)
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(90, DENOMINATION)
    );
    assert_eq!(app.wrap().query_all_balances(bidder).unwrap(), vec![]);
}

#[test]
fn multiple_bidders() {
    let owner = Addr::unchecked("owner");
    let alex = Addr::unchecked("alex");
    let ann = Addr::unchecked("ann");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alex, coins(100, DENOMINATION))
            .unwrap();
        router
            .bank
            .init_balance(storage, &ann, coins(100, DENOMINATION))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None).unwrap();

    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(resp.bid, None);

    contract
        .bid(&mut app, &alex, &coins(10, DENOMINATION))
        .unwrap();

    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(
        resp.bid,
        Some(AddrAmount {
            addr: alex.clone(),
            amount: Uint128::new(9)
        })
    );

    contract
        .bid(&mut app, &ann, &coins(20, DENOMINATION))
        .unwrap();

    let resp = contract.query_highest_bid(&app).unwrap();
    assert_eq!(
        resp.bid,
        Some(AddrAmount {
            addr: ann,
            amount: Uint128::new(18)
        })
    );

    let err = contract
        .bid(&mut app, &alex, &coins(10, DENOMINATION))
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::InsufficientBid {
            min: Uint128::new(18)
        }
    );
}

#[test]
fn close_bids() {
    let owner = Addr::unchecked("owner");
    let alex = Addr::unchecked("alex");
    let ann = Addr::unchecked("ann");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alex, coins(100, DENOMINATION))
            .unwrap();
        router
            .bank
            .init_balance(storage, &ann, coins(100, DENOMINATION))
            .unwrap();
    });
    let code_id = BiddingContract::store_code(&mut app);

    let contract =
        BiddingContract::instantiate(&mut app, code_id, &owner, "Bidding contract", None).unwrap();

    let resp = contract.query_winner(&app).unwrap();
    assert_eq!(resp.winner, None);

    let err = contract.close(&mut app, &alex).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized);

    contract
        .bid(&mut app, &alex, &coins(10, DENOMINATION))
        .unwrap();

    contract
        .bid(&mut app, &ann, &coins(20, DENOMINATION))
        .unwrap();

    contract.close(&mut app, &owner).unwrap();

    let resp = contract.query_winner(&app).unwrap();
    assert_eq!(resp.winner, Some(ann.clone()));
}
