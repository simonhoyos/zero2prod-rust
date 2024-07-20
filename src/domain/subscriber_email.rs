use validator::Validate;

#[derive(Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String,
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let data = Self { email: s };
        match data.validate() {
            Ok(_) => Ok(data),
            Err(_) => Err(format!("{} is not a valid subscriber email.", data.email)),
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use quickcheck::{Arbitrary, Gen};

    use crate::domain::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();

        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_mission_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();

        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();

        assert_err!(SubscriberEmail::parse(email));
    }

    /// alternatives for property-based testing
    /// quickcheck vs proptest
    #[derive(Debug, Clone)]
    struct ValidEmailFixtures(pub String);

    impl Arbitrary for ValidEmailFixtures {
        fn arbitrary(_g: &mut Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixtures) -> bool {
        // dbg!(&valid_email.0); cargo test valid_email --nocapture
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
