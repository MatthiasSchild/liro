use regex::Regex;

pub fn is_secure_password(password: &str) -> bool {
    let re = Regex::new("[a-z]+").unwrap();
    if !re.is_match(password) {
        return false;
    }
    let re = Regex::new("[A-Z]+").unwrap();
    if !re.is_match(password) {
        return false;
    }
    let re = Regex::new("[0-9]+").unwrap();
    if !re.is_match(password) {
        return false;
    }
    let re = Regex::new("[ !\"#$%&'()*+,-./:;<=>?@[\\\\]^_`{|}~]+").unwrap();
    if !re.is_match(password) {
        return false;
    }

    true
}
