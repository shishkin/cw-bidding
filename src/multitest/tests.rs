use cosmwasm_std::Addr;
use cw_multi_test::App;

use crate::state::STATE;

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
