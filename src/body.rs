use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
};

/// A single contiguous block of [`CommitMessage`] text
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Body<'a> {
    text: Cow<'a, str>,
}

impl Body<'_> {
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
    /// Create from a Cow<_, str>
    ///
    /// # Example
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use mit_commit::Body;
    ///
    /// let expected = "a string";
    /// let input = Cow::from(expected);
    /// assert_eq!(Body::from(input).to_string(), expected)
    /// ```
    fn from(body: Cow<'a, str>) -> Self {
        Self { text: body }
    }
}
impl<'a> From<&'a str> for Body<'a> {
    fn from(body: &'a str) -> Self {
        Self::from(Cow::Borrowed(body))
    }
}

impl From<String> for Body<'_> {
    fn from(body: String) -> Self {
        Self::from(Cow::from(body))
    }
}

impl From<Body<'_>> for String {
    fn from(body: Body<'_>) -> Self {
        body.text.into()
    }
}

impl<'a> From<Body<'a>> for Cow<'a, str> {
    /// Convert to a Cow<_, str>
    ///
    /// # Example
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use mit_commit::Body;
    ///
    /// let expected = Cow::from("a string");
    /// let input = Body::from(expected.clone());
    /// assert_eq!(Cow::from(input), expected)
    /// ```
    fn from(body: Body<'a>) -> Self {
        body.text
    }
}

impl Display for Body<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
