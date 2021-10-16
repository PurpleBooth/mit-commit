use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// A single contiguous block of [`CommitMessage`] text
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Body {
    text: String,
}

impl Body {
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

impl From<&str> for Body {
    fn from(body: &str) -> Self {
        Self {
            text: String::from(body),
        }
    }
}

impl From<String> for Body {
    fn from(body: String) -> Self {
        Self { text: body }
    }
}

impl From<Body> for String {
    fn from(body: Body) -> Self {
        body.text
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
