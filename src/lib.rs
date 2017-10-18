
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    
    // Perform a simple regex match to validate email
    static ref EMAIL_REGEX: regex::Regex = Regex::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
}


/// Validates Email Format using regex.
/// See [this](https://www.w3.org/TR/html5/forms.html#valid-e-mail-address) for more information
pub fn validate_email(email: &String) -> bool {
    EMAIL_REGEX.is_match(email.as_str())
}

mod tests {

    #[test]
    fn good_email() {
        let mut email = Vec::new();

        email.push(String::from("ishanjain28@gmail.com"));
        email.push(String::from("abc+xyz@google.com"));

        for e in email {
            assert_eq!(true, super::validate_email(&e));
        }
    }

    #[test]
    fn bad_email() {
        let email = String::from("ishanjain.gmail.com");

        assert_eq!(false, super::validate_email(&email));
    }
}
