use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

const FORBIDDEN_CHARACTERS: [char; 11] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '[', ']'];

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let contains_forbidden_characters = s.chars().any(|g| FORBIDDEN_CHARACTERS.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    use super::FORBIDDEN_CHARACTERS;

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "é".repeat(257);

        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();

        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &FORBIDDEN_CHARACTERS {
            let name = name.to_string();

            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        // let name = "Ursule Le Guin".to_string();
        let name = "é".repeat(256);

        assert_ok!(SubscriberName::parse(name));
    }
}
