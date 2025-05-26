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
    /// # Arguments
    ///
    /// * `additional` - The body to append to this one
    ///
    /// # Returns
    ///
    /// A new body with the content of both bodies separated by a newline
    ///
    /// # Examples
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

    /// Checks if this [`Body`] is empty
    ///
    /// # Returns
    ///
    /// `true` if the body is empty, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Body;
    ///
    /// assert_eq!(Body::from("").is_empty(), true);
    /// assert_eq!(Body::from("not empty").is_empty(), false);
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<'a> From<Cow<'a, str>> for Body<'a> {
    /// Create a Body from a Cow<_, str>
    ///
    /// # Arguments
    ///
    /// * `body` - The string content to create the body from
    ///
    /// # Returns
    ///
    /// A new Body containing the provided string
    ///
    /// # Examples
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
    /// Create a Body from a string slice
    ///
    /// # Arguments
    ///
    /// * `body` - The string slice to create the body from
    ///
    /// # Returns
    ///
    /// A new Body containing the provided string
    fn from(body: &'a str) -> Self {
        Self::from(Cow::Borrowed(body))
    }
}

impl From<String> for Body<'_> {
    /// Create a Body from a String
    ///
    /// # Arguments
    ///
    /// * `body` - The string to create the body from
    ///
    /// # Returns
    ///
    /// A new Body containing the provided string
    fn from(body: String) -> Self {
        Self::from(Cow::from(body))
    }
}

impl From<Body<'_>> for String {
    /// Convert a Body to a String
    ///
    /// # Arguments
    ///
    /// * `body` - The body to convert
    ///
    /// # Returns
    ///
    /// A String containing the body's text
    fn from(body: Body<'_>) -> Self {
        body.text.into()
    }
}

impl<'a> From<Body<'a>> for Cow<'a, str> {
    /// Convert a Body to a Cow<_, str>
    ///
    /// # Arguments
    ///
    /// * `body` - The body to convert
    ///
    /// # Returns
    ///
    /// A Cow<_, str> containing the body's text
    ///
    /// # Examples
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
    /// Format the Body for display
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// A Result indicating whether the operation succeeded
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_string_conversion_from_str() {
        let body = Body::from("Example Body");

        assert_eq!(
            String::from(body),
            String::from("Example Body"),
            "Body should convert to the correct string when created from a str"
        );
    }

    #[test]
    fn test_string_conversion_from_string() {
        let body = Body::from(String::from("Example Body"));

        assert_eq!(
            String::from(body),
            String::from("Example Body"),
            "Body should convert to the correct string when created from a String"
        );
    }

    #[test]
    fn test_display_implementation() {
        let body = Body::from("Example Body");

        assert_eq!(
            format!("{body}"),
            "Example Body",
            "Display implementation should format the body correctly"
        );
    }

    #[test]
    fn test_append_body_fragments() {
        assert_eq!(
            Body::from(indoc!(
                "
                Example 1
                Example 2"
            )),
            Body::from("Example 1").append(&Body::from("Example 2")),
            "Appending bodies should create a new body with content separated by newline"
        );
    }

    #[test]
    fn test_is_empty_with_empty_body() {
        assert!(
            Body::from("").is_empty(),
            "Empty body should be identified as empty"
        );
    }

    #[test]
    fn test_is_empty_with_non_empty_body() {
        assert!(
            !Body::from("something").is_empty(),
            "Non-empty body should not be identified as empty"
        );
    }
}
