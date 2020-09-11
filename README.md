# edn-derive

Edn derive procedural macros for (De)Serialization.

**This library still is `pre-alpha`**.

## Usage

```toml
edn-derive = "0.3.2"
```

## Example

Serialization
```rust
use edn_derive::Serialize;

#[derive(Serialize)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: 290000,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :age 290000, }"
    );
}
```

Deserialization
```rust
use edn_derive::Deserialize;
use edn_rs::EdnError;

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() -> Result<(), EdnError> {
    let edn_person = "{ :name \"joana\", :age 290000, }";

    let person: Person = edn_rs::from_str(edn_person)?;

    assert_eq!(
        person,
        Person {
            name: "joana".to_string(),
            age: 290000,
        }
    );

    Ok(())
}
```
