use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
};

use nom::{
    bytes::complete::take_till1,
    character::is_newline,
    InputTakeAtPosition,
};

/// A single contiguous block of [`CommitMessage`] text
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Body<'a> {
    text: Cow<'a, str>,
}

impl<'a> Body<'a> {
    pub fn parser<I, F, E: nom::error::ParseError<I>>() -> impl FnMut(I) -> Result<(I, I), nom::Err<E>>
        where
            I: InputTakeAtPosition<Item=u8>,
            F: Fn(<I as InputTakeAtPosition>::Item) -> bool,
    {
        take_till1(is_newline)
    }

    /// Append one [`Body`] onto another
    ///
    /// This is for concatenating multiple [`Bodies`] together
    ///
    /// # Example
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::Body;
    ///
    /// assert_eq!(
    ///     Body::from(indoc!(
    ///         "
    ///         Example 1
    ///         Example 2"
    ///     )),
    ///     Body::from("Example 1").append(&Body::from("Example 2"))
    /// )
    /// ```
    #[must_use]
    pub fn append(&self, additional: &Self) -> Self {
        Self::from(format!("{}\n{}", self.text, additional.text))
    }

    /// Is this [`Body`] empty
    ///
    /// An empty [`Body`] usually indicate a paragraph break in a
    /// [`CommitMessage`] so it's handy to be able to see them.
    ///
    /// # Example
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::Body;
    ///
    /// assert_eq!(Body::from("").is_empty(), true)
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<'a> From<Cow<'a, str>> for Body<'a> {
    fn from(body: Cow<'a, str>) -> Self {
        Self { text: body }
    }
}

impl<'a> From<&'a str> for Body<'a> {
    fn from(body: &'a str) -> Self {
        Self::from(Cow::Borrowed(body))
    }
}

impl<'a> From<String> for Body<'a> {
    fn from(body: String) -> Self {
        Self::from(Cow::from(body))
    }
}

impl<'a> From<Body<'a>> for String {
    fn from(body: Body<'_>) -> Self {
        body.text.into()
    }
}

impl<'a> From<Body<'a>> for Cow<'a, str> {
    fn from(body: Body<'a>) -> Self {
        body.text
    }
}

impl<'a> Display for Body<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
