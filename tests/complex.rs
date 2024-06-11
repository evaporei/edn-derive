#![allow(dead_code)]

use edn_derive::{Deserialize, Serialize};
use edn_rs::EdnError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum AccountType {
    Basic,
    Premium,
    PremiumPlus,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Account {
    crux__db___id: String,
    account___amount: usize,
    account_type: AccountType,
}

fn main() -> Result<(), EdnError> {
    let account = Account {
        crux__db___id: "123".to_string(),
        account___amount: 42,
        account_type: AccountType::PremiumPlus,
    };

    let account_edn_str =
        "{ :crux.db/id \"123\", :account/amount 42, :account-type :account-type/premium-plus, }";

    assert_eq!(edn_rs::to_string(account), account_edn_str);

    let account: Account = edn_rs::from_str(account_edn_str)?;

    assert_eq!(
        account,
        Account {
            crux__db___id: "123".to_string(),
            account___amount: 42,
            account_type: AccountType::PremiumPlus,
        }
    );

    Ok(())
}
