use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct EZCard {
    title: String,
    body: String,
}

fn main() {
    let my_card = EZCard {
        title: String::from("GitHub Achievement"),
        body: String::from("Unlocked the Starstruck badge today!"),
    };
    
    let j = serde_json::to_string(&my_card).unwrap();
    println!("Serialized Card: {}", j);
}
