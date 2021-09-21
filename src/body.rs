use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// A single contiguous block of `CommitMessage` text
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Body {
    text: String,
}

impl Body {
    /// Append one body onto another
    ///
    /// This is for concatenating multiple bodies together
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
    pub fn append(&self, additional: &Body) -> Body {
        Body::from(format!("{}\n{}", self.text, additional.text))
    }

    /// Is this body empty
    ///
    /// Empty bodies usually indicate a paragraph break in commit messages so
    /// it's handy to be able to see them.
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
        Body {
            text: String::from(body),
        }
    }
}

impl From<String> for Body {
    fn from(body: String) -> Self {
        Body { text: body }
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

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::Body;

    #[test]
    fn it_can_give_me_it_as_a_string_from_a_str() {
        let body = Body::from("Example Body");

        assert_eq!(String::from(body), String::from("Example Body"));
    }

    #[test]
    fn it_can_give_me_it_as_a_string_from_a_string() {
        let body = Body::from(String::from("Example Body"));

        assert_eq!(String::from(body), String::from("Example Body"));
    }

    #[test]
    fn it_implements_display() {
        let body = Body::from("Example Body");

        assert_eq!(format!("{}", body), "Example Body");
    }

    #[test]
    fn it_can_append_another_body_fragment() {
        assert_eq!(
            Body::from(indoc!(
                "
                Example 1
                Example 2"
            )),
            Body::from("Example 1").append(&Body::from("Example 2"))
        );
    }

    #[test]
    fn it_can_tell_me_if_it_is_empty() {
        assert!(Body::from("").is_empty());
    }

    #[test]
    fn it_can_tell_me_if_it_is_full() {
        assert!(!Body::from("something").is_empty());
    }
}
