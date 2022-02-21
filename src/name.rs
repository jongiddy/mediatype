use super::parse::*;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// A media-type name.
///
/// A valid name has the following requirements:
///
/// - The first character must be an alphabet or a number.
/// - The subsequent characters must be alphabets, numbers or `!#$&-^_.+%*'`.
/// - Length of the name can not exceed 127 bytes.
#[derive(Debug, Copy, Clone)]
pub struct Name<'a>(pub(crate) &'a str);

impl<'a> Name<'a> {
    /// Constructs a `Name`.
    ///
    /// If the string is not valid as a name, returns `None`.
    pub fn new(s: &'a str) -> Option<Self> {
        if is_restricted_name(s) {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Returns the underlying string.
    pub fn as_str(&self) -> &str {
        self.0
    }

    /// The maximum byte length of a name.
    pub const MAX_LENGTH: usize = 127;
}

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> AsRef<str> for Name<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'a> PartialEq for Name<'a> {
    fn eq(&self, other: &Name) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'a> Eq for Name<'a> {}

impl<'a> PartialOrd for Name<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Name<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .to_ascii_lowercase()
            .cmp(&other.0.to_ascii_lowercase())
    }
}

impl<'a> Hash for Name<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl<'a> PartialEq<String> for Name<'a> {
    fn eq(&self, other: &String) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialOrd<String> for Name<'a> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl<'a> PartialEq<str> for Name<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialEq<&str> for Name<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialOrd<str> for Name<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl<'a> PartialOrd<&str> for Name<'a> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}
