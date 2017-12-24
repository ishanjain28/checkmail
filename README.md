# checkmail
A Simple Email Validation Library in Rust. See [this](https://www.w3.org/TR/html51/sec-forms.html#valid-e-mail-address) for more information.

# Usage

    checkmail = "0.1.1"

# Example

    extern crate checkmail;

    fn main() {
        let email = String::from("abc+xyz@example.com");

        let result = checkmail::validate_email(&email);

        if result {
            println!("Email is Valid");
        } else {
            println!("Email is invalid");
        }
    }

# License 

MIT
