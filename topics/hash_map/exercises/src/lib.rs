use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut hash: HashMap<String,u32> = HashMap::new();
    hash.insert(address, amount);
    return hash;
}
