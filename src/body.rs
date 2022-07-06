use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{map, not, peek, recognize, rest},
    error::ParseError,
    sequence::{pair, terminated},
    IResult,
};

/// A single contiguous block of [`CommitMessage`] text
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Body<'a> {
    text: Cow<'a, str>,
}

impl<'a> Body<'a> {
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

    /// Build a parser for bodies
    pub fn parser<E: 'a + ParseError<&'a str>>(
        comment_char: char,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Body<'a>, E> + 'a {
        map(
            recognize(pair(
                recognize(not(char(comment_char))),
                alt((
                    terminated(
                        recognize(pair(take_until1("\n"), tag("\n"))),
                        peek(char(comment_char)),
                    ),
                    recognize(pair(take_until("\n\n"), tag("\n\n"))),
                    recognize(pair(take(1_usize), rest)),
                )),
            )),
            |raw_body: &'a str| -> Body<'a> { Cow::from(raw_body).into() },
        )
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
