use std::{path::PathBuf};

use lib::user_agent::{Type, UserAgent};

fn main() {
    // uses a custom json file for ua configuraton.
    let ua = UserAgent::new(Some(PathBuf::from("./examples/custom/custom-ua.json"))).unwrap();
    for _ in 0..20 {
        let result = ua.generate_ua(&Type::Desktop).unwrap();
        println!("{}", result);
    }
}
