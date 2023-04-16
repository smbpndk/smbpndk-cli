use regex::Regex;

pub fn email_validation(input: &str) -> Result<(), &'static str> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    if email_regex.is_match(input) {
        Ok(())
    } else {
        Err("Username must be an email address")
    }
}
