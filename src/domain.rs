//! src/domain.rs

use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName
}


impl SubscriberName {

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    pub fn parse(s: String) -> SubscriberName{

        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 255;

        let mut forbidden_chars: HashSet<char> = HashSet::new();
        forbidden_chars.insert('/');
        forbidden_chars.insert('\\');
        forbidden_chars.insert('?');
        forbidden_chars.insert('<');

        let contains_forbidden = s.chars().any(|g| forbidden_chars.contains(&g));
        if is_too_long || is_empty_or_whitespace || contains_forbidden {
            panic!("{} is not a valid subscriber name.", s);
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
