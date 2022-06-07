use crate::user_agent::Probability;
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    time::{SystemTime, UNIX_EPOCH},
};

// apply_name_format apply format using a map data
pub fn apply_named_format(format: &str, data: &HashMap<String, String>) -> String {
    let mut final_string = Cow::from(format);

    for (key, val) in data.iter() {
        let pattern = format!("<{}>", key);

        final_string = final_string.replace(&pattern, &val).into();
    }

    final_string.into()
}

// get a random value from the vector
pub fn random_vector_element<T>(vals: &Vec<T>) -> Option<&T> {
    let i = fastrand::usize(..vals.len());
    vals.get(i)
}

// get a random value from a hashmap
pub fn random_hashmap_element<T, U>(h: &HashMap<T, U>) -> Option<&U>
where
    T: std::clone::Clone + Eq + Hash,
{
    let keys: Vec<T> = h.keys().cloned().collect();

    let i = fastrand::usize(..keys.len());
    let key = match keys.get(i) {
        Some(v) => v,
        None => return None,
    };

    match h.get(key) {
        Some(v) => Some(v),
        None => return None,
    }
}

pub fn calculate_probability_limit<T>(vals: Vec<&T>) -> f64
where
    T: Probability,
{
    let mut limit: f64 = 0.0;

    for val in vals {
        limit += val.get_probability();
    }

    limit
}

// random_weighted get a randomly weighted value by using each element match probablity,
pub fn random_weighted<T>(vals: Vec<&T>, limit: f64) -> Option<&T>
where
    T: Probability,
{
    // return none when empty vec
    let size = vals.len();
    if size < 1 {
        return None;
    }

    // calculate probable match
    let r = fastrand::f64() * limit;
    let mut store: f64 = 0.0;

    for val in vals {
        store += val.get_probability();

        // skip when probability is higher than store
        if r > store {
            continue;
        }

        // return the value
        return Some(val);
    }

    // if not found, return none
    None
}

// Set seed of randomness on thread local
pub fn reseed_randomness() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // safely convert u128 random into u64 random
    let seed: u64 = (now % (u64::MAX as u128)) as u64;
    fastrand::seed(seed);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_correctly_do_modulus_for_converting() {
        let now = (u64::MAX as u128) + 1;
        let seed: u64 = (now % (u64::MAX as u128)) as u64;
        assert_eq!(1, seed);
    }
}
