use cosmwasm_std::Addr;
use cw_multi_test::App;

use super::BiddingContract;

#[test]
fn instantiate() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");
    let code_id = BiddingContract::store_code(&mut app);

    let _contract =
        BiddingContract::instantiate(&mut app, code_id, &sender, "Counting contract").unwrap();
}
