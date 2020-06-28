use crate::body::Body;
use std::convert::TryFrom;
use thiserror::Error;

/// A `Trailer` you might see a in a `CommitMessage`, for example 'Co-authored-by: Billie Thompson <billie@example.com>'
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Trailer {
    key: String,
    value: String,
}

impl Trailer {
    /// Create a new `Trailer`
    ///
    /// This creates a new element that represents the sort of `Trailers` you get at the end of commits
    ///
    /// For example there's `Co-authored-by`, `Relates-to`, and `Signed-off-by`
    ///
    /// # Example
    ///
    /// ```
    /// use mit_commit::{Body, Trailer};
    /// use std::convert::TryFrom;
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

    /// Get the key of the `Trailer`
    ///
    /// # Example
    ///
    /// ```
    /// use mit_commit::{Body, Trailer};
    /// use std::convert::TryFrom;
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by", "#124").get_key(),
    ///     "Co-authored-by"
    /// )
    /// ```
    #[must_use]
    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    /// Get the value of the `Trailer`
    ///
    /// # Example
    ///
    /// ```
    /// use mit_commit::{Body, Trailer};
    /// use std::convert::TryFrom;
    /// assert_eq!(Trailer::new("Co-authored-by", "#124").get_value(), "#124")
    /// ```
    #[must_use]
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl From<Trailer> for String {
    fn from(trailer: Trailer) -> Self {
        format!("{}: {}", trailer.key, trailer.value)
    }
}

impl TryFrom<Body> for Trailer {
    type Error = Error;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let body_value = String::from(body.clone());
        let value_and_key: Vec<&str> = body_value.split(": ").collect();

        let key = value_and_key
            .get(0)
            .ok_or_else(|| Error::NotATrailer(body.clone()))?;
        let value = value_and_key.get(1).ok_or(Error::NotATrailer(body))?;

        Ok(Trailer::new(key, value))
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no colon in body line, {0} is not a trailer")]
    NotATrailer(Body),
}

#[cfg(test)]
mod tests {
    use super::Trailer;
    use crate::body::Body;
    use pretty_assertions::assert_eq;
    use std::convert::TryFrom;

    #[test]
    fn it_can_tell_me_its_key() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(trailer.get_key(), String::from("Relates-to"))
    }

    #[test]
    fn it_can_tell_me_its_value() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(trailer.get_value(), String::from("#128"))
    }

    #[test]
    fn it_can_give_me_itself_as_a_string() {
        let trailer = Trailer::new("Relates-to", "#128");

        assert_eq!(String::from(trailer), String::from("Relates-to: #128"))
    }

    #[test]
    fn can_generate_itself_from_body() {
        let trailer = Trailer::try_from(Body::from("Relates-to: #128"));

        assert_eq!(
            String::from(trailer.expect("Could not parse from string")),
            String::from("Relates-to: #128")
        )
    }
}
