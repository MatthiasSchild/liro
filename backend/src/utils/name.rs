pub fn valid_name(name: &str) -> bool {
    // TODO check if it only consists of letters
    name.len() >= 3 && name.len() < 16
}
