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
    pub fn new(key: &str, value: &str) -> Self {
        Self {
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
        Self::Body(Body::from(trailer))
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

        Ok(Self::new(key, value))
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
    fn new_not_a_trailer(body: &Body) -> Self {
        let text: String = body.clone().into();
        Self::NotATrailer(text.clone(), (0, text.len()))
    }
}
