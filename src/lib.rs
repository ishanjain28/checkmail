
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate lettre;

use regex::Regex;
use lettre::{SmtpTransport, mechaisn, smtp};
use std::net;


lazy_static! {
    
    // Perform a simple regex match to validate email
    static ref EMAIL_REGEX: regex::Regex = Regex::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
}


/// Validates Email Format using regex.
/// See [this](https://www.w3.org/TR/html5/forms.html#valid-e-mail-address) for more information
pub fn validate_format(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn validate_host(email: &str) -> bool {

    let email = email.split("@").collect::<Vec<&str>>();

    if email.len() != 2 {
        return false;
    }
    let email_host = email[1];

    match SmtpTransport::simple_builder(email_host.to_string()) {
        Ok(v) => {
            println!("{}", email_host);

            let transport = v.hello_name("checkmail")
                .authentication_mechanism(smtp::authentication::Mechanism::Plain)
                .build();

            let res = transport.get_ehlo();








            true
        }
        Err(_) => false,
    }


    //    SmtpTransport::simple_builder(email_host.to_string()) {
    //      Ok(v) => {
    // true
    //    }
    //   Err(_) => false,
    //}
}

mod tests {

    #[test]
    fn validate_format_test() {
        let good_emails: [&str; 17] = [
            "rick@madscientist.com",
            "abc+xyz@google.com",
            "abc@gmail.com",
            "qq@twitter.com",
            "xyz@totallyfakeemail.com",
            "email@example.com",
            "firstname.lastname@example.com",
            "email@subdomain.example.com",
            "firstname+lastname@example.com",
            "email@123.123.123.123",
            "1234567890@example.com",
            "email@example-one.com",
            "_______@example.com",
            "email@example.name",
            "email@example.museum",
            "email@example.co.jp",
            "firstname-lastname@example.com",
        ];

        let bad_emails: [&str; 9] = [
            "           plainaddress",
            "#@%^%#$@#$@#.com",
            "@example.com",
            "Joe Smith <email@example.com>",
            "email.example.com",
            "email@example@example.com",
            "あいうえお@example.com",
            "email@example.com (Joe Smith)",
            "email@-example.com",
        ];


        for e in good_emails.iter() {
            assert_eq!(true, super::validate_format(&e));
        }
        for e in bad_emails.iter() {
            assert_eq!(false, super::validate_format(&e));
        }


    }

    #[test]
    fn good_email_host() {
        let good_hosts: [&str; 3] = ["help@gmail.com", "info@twitter.com", "hacked@yahoo.com"];

        for e in good_hosts.iter() {
            let email: String = e.split("@").collect();
            assert_eq!(true, super::validate_host(&e));
        }
    }
}
