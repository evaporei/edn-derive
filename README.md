# edn-derive

Edn derive procedural macros for (De)Serialization.

**This library still is `beta` but very close to stable**.

## Usage

```toml
[dependencies]
edn-derive = "0.5.0"
```

## Example

Serialization
```rust
use edn_derive::Serialize;

#[derive(Serialize)]
enum Kind {
    Cool,
    Chill,
    Pirate,
}

#[derive(Serialize)]
pub struct Person {
    name: String,
    age: usize,
    kind: Kind,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: 290000,
        kind: Kind::Chill,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :age 290000, :kind :kind/chill, }"
    );
}
```

Deserialization
```rust
use edn_derive::Deserialize;
use edn_rs::EdnError;

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
enum Kind {
    Cool,
    Chill,
    Pirate,
}

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
pub struct Person {
    name: String,
    age: usize,
    kind: Kind,
}

fn main() -> Result<(), EdnError> {
    let edn_person = "{ :name \"joana\", :age 290000, :kind :kind/pirate, }";

    let person: Person = edn_rs::from_str(edn_person)?;

    assert_eq!(
        person,
        Person {
            name: "joana".to_string(),
            age: 290000,
            kind: Kind::Pirate,
        }
    );

    Ok(())
}
```

With more complexity using `.`, `-` and `/` on EDN conversions:

```rust
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
```
