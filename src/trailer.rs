use std::{
    borrow::Cow,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::{Fragment, body::Body};

/// A [`Trailer`] you might see a in a [`CommitMessage`], for example
/// 'Co-authored-by: Billie Thompson <billie@example.com>'
#[derive(Debug, Clone, Eq, Ord, PartialOrd)]
pub struct Trailer<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> Trailer<'a> {
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
    ///     Trailer::new("Co-authored-by".into(), "#124".into()),
    ///     Trailer::try_from(Body::from("Co-authored-by: #124"))
    ///         .expect("There should have been a trailer in that body component")
    /// )
    /// ```
    #[must_use]
    pub const fn new(key: Cow<'a, str>, value: Cow<'a, str>) -> Self {
        Self { key, value }
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
    ///     Trailer::new("Co-authored-by".into(), "#124".into()).get_key(),
    ///     "Co-authored-by"
    /// )
    /// ```
    #[must_use]
    pub fn get_key(&self) -> String {
        format!("{}", self.key)
    }

    /// Get the value of the [`Trailer`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by".into(), "#124".into()).get_value(),
    ///     "#124"
    /// )
    /// ```
    #[must_use]
    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

impl PartialEq for Trailer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value.trim_end() == other.value.trim_end()
    }
}

impl Hash for Trailer<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.trim_end().hash(state);
    }
}

impl From<Trailer<'_>> for String {
    fn from(trailer: Trailer<'_>) -> Self {
        format!("{}: {}", trailer.key, trailer.value)
    }
}

impl<'a> From<Trailer<'a>> for Fragment<'a> {
    fn from(trailer: Trailer<'_>) -> Self {
        let trailer: String = trailer.into();
        Body::from(trailer).into()
    }
}

impl<'a> TryFrom<Body<'a>> for Trailer<'a> {
    type Error = Error;

    fn try_from(body: Body<'a>) -> Result<Self, Self::Error> {
        let content: String = body.clone().into();
        let mut value_and_key = content.split(": ").map(ToString::to_string);

        let key: String = value_and_key
            .next()
            .ok_or_else(|| Error::new_not_a_trailer(&body))?;

        let value: String = value_and_key
            .next()
            .ok_or_else(|| Error::new_not_a_trailer(&body))?;

        Ok(Trailer::new(key.into(), value.into()))
    }
}

/// Errors in parsing potential trailers
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// When the given fragment is not a trailer
    #[error("not a trailer")]
    #[diagnostic(url(docsrs), code(mit_commit::trailer::error::not_atrailer))]
    NotATrailer(
        #[source_code] String,
        #[label("no colon in body line")] (usize, usize),
    ),
}

impl Error {
    fn new_not_a_trailer(body: &Body<'_>) -> Self {
        let text: String = body.clone().into();
        Self::NotATrailer(text.clone(), (0, text.len()))
    }
}
