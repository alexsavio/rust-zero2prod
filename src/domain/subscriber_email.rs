use validator::Validate;

#[derive(Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String,
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let potential_email = SubscriberEmail { email: s.clone() };
        match potential_email.validate() {
            Ok(_) => Ok(potential_email),
            Err(e) => Err(format!(
                "{} is not a valid email address. Error: {:?}",
                s, e
            )),
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.email.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "name.example.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@example.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
