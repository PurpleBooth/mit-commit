use std::{
    convert::TryFrom,
    hash::{Hash, Hasher},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::{body::Body, Fragment};

/// A [`Trailer`] you might see a in a [`CommitMessage`], for example
/// 'Co-authored-by: Billie Thompson <billie@example.com>'
#[derive(Debug, Clone, Eq, Ord, PartialOrd)]
pub struct Trailer {
    key: String,
    value: String,
}

impl Trailer {
    /// Create a new [`Trailer`]
    ///
    /// This creates a new element that represents the sort of [`Trailers`] you
    /// get at the end of commits
    ///
    /// For example there's `Co-authored-by`, `Relates-to`, and `Signed-off-by`
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by", "#124"),
    ///     Trailer::try_from(Body::from("Co-authored-by: #124"))
    ///         .expect("There should have been a trailer in that body component")
    /// )
    /// ```
    #[must_use]
    pub fn new(key: &str, value: &str) -> Trailer {
        Trailer {
            key: String::from(key),
            value: String::from(value),
        }
    }

    /// Get the key of the [`Trailer`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by", "#124").get_key(),
    ///     "Co-authored-by"
    /// )
    /// ```
    #[must_use]
    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    /// Get the value of the [`Trailer`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(Trailer::new("Co-authored-by", "#124").get_value(), "#124")
    /// ```
    #[must_use]
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl PartialEq for Trailer {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value.trim_end() == other.value.trim_end()
    }
}

impl Hash for Trailer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.trim_end().hash(state);
    }
}

impl From<Trailer> for String {
    fn from(trailer: Trailer) -> Self {
        format!("{}: {}", trailer.key, trailer.value)
    }
}

impl From<Trailer> for Fragment {
    fn from(trailer: Trailer) -> Self {
        let trailer: String = trailer.into();
        Fragment::Body(Body::from(trailer))
    }
}

impl TryFrom<Body> for Trailer {
    type Error = Error;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let body_value = String::from(body.clone());
        let value_and_key: Vec<&str> = body_value.split(": ").collect();

        let key = value_and_key
            .get(0)
            .ok_or_else(|| Error::new_not_a_trailer(&body))?;
        let value = value_and_key
            .get(1)
            .ok_or_else(|| Error::new_not_a_trailer(&body))?;

        Ok(Trailer::new(key, value))
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("not a trailer")]
    #[diagnostic(url(docsrs), code(mit_commit::trailer::error::not_atrailer))]
    NotATrailer(
        #[source_code] String,
        #[label("no colon in body line")] (usize, usize),
    ),
}

impl Error {
    fn new_not_a_trailer(body: &Body) -> Error {
        let text: String = body.clone().into();
        Error::NotATrailer(text.clone(), (0, text.len()))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    };

    use super::Trailer;
    use crate::{body::Body, Fragment};

    #[test]
    fn it_can_tell_me_its_key() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(trailer.get_key(), String::from("Relates-to"));
    }

    #[test]
    fn it_can_tell_me_its_value() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(trailer.get_value(), String::from("#128"));
    }

    #[test]
    fn it_does_not_take_trailing_whitespace_into_account_in_equality_checks() {
        let a = Trailer::new("Relates-to", "#128\n");
        let b = Trailer::new("Relates-to", "#128");

        assert_eq!(a, b);
    }

    #[test]
    fn it_does_not_take_trailing_whitespace_into_account_in_hashing() {
        let mut hasher_a = DefaultHasher::new();
        Trailer::new("Relates-to", "#128\n").hash(&mut hasher_a);

        let mut hasher_b = DefaultHasher::new();
        Trailer::new("Relates-to", "#128").hash(&mut hasher_b);

        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn it_can_give_me_itself_as_a_string() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(String::from(trailer), String::from("Relates-to: #128"));
    }

    #[test]
    fn can_generate_itself_from_body() {
        let trailer = Trailer::try_from(Body::from("Relates-to: #128"));

        assert_eq!(
            String::from(trailer.expect("Could not parse from string")),
            String::from("Relates-to: #128")
        );
    }

    #[test]
    fn it_preserves_preceding_whitespace() {
        let trailer = Trailer::try_from(Body::from("Relates-to:      #128\n"));

        assert_eq!(
            String::from(trailer.expect("Could not parse from string")),
            String::from("Relates-to:      #128\n")
        );
    }

    #[test]
    fn can_generate_from_body() {
        let trailer = Trailer::new("Relates-to", "#128");
        let body: Fragment = Fragment::from(trailer);

        assert_eq!(body, Fragment::Body(Body::from("Relates-to: #128")));
    }
}
