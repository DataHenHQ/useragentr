use lib::user_agent::{Type, UserAgent};

fn main() {
  // uses the default user agent configuration, by specifying None to the path parameter
    let ua = UserAgent::new(None).unwrap();
    for _ in 0..20 {
        let result = ua.generate_ua(&Type::Desktop).unwrap();
        println!("{}", result);
    }
}
