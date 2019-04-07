use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

fn main() {
    let mut db = sfsdb::new("db");

    let u = TestData {
        first_entry: "some string".to_string(),
        second_entry: 1234,
    };

    db.save("some key", &u).unwrap();
    db.save("other key", &u).unwrap();

    assert_eq!(db.exists("some key"), true);
    assert_eq!(u, db.load::<TestData>("some key").unwrap());
    assert_eq!(u, db.load::<TestData>("other key").unwrap());
}
