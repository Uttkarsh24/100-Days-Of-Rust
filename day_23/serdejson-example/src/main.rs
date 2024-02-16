use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]
struct Dog {
    name: String,
    year_born: i32,
    onwer: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    deserialize_test();
    serialize_test();
}

fn serialize_test() {
    let dog01 = Dog {
        name: "Siro".to_string(),
        year_born: 2023,
        onwer: DogOwner {
            first_name: "Anmol".to_string(),
            last_name: "Anand".to_string(),
        },
    };
    let dog_ser = to_string_pretty(&dog01);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

fn deserialize_test() {
    let json_string = r#"
    {
        "name": "Siro",
        "year_born": 2023,
        "onwer": {
            "first_name": "Anmol",
            "last_name": "Anand"
        }
    }
    "#;

    let dog_deser = from_str::<Dog>(json_string);
    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
