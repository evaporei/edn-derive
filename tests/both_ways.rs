use edn_derive::{Deserialize, Serialize};
use edn_rs::EdnError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), EdnError> {
    let point = Point { x: 1, y: 2 };

    let serialized = edn_rs::to_string(point.clone());
    let deserialized: Point = edn_rs::from_str(&serialized)?;

    assert_eq!(point, deserialized);

    Ok(())
}
