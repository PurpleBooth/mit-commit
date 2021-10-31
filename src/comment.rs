use std::borrow::Cow;

/// A single comment from a `CommitMessage`
#[derive(Debug, PartialEq, Clone)]
pub struct Comment<'a> {
    comment: Cow<'a, str>,
}

impl<'a> Comment<'a> {
    /// Append one [`Comment`] onto another
    ///
    /// This is for concatenating multiple [`Comment`] together
    ///
    /// # Example
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::Comment;
    ///
    /// assert_eq!(
    ///     Comment::from(indoc!(
    ///         "
    ///         Example 1
    ///         Example 2"
    ///     )),
    ///     Comment::from("Example 1").append(&Comment::from("Example 2"))
    /// )
    /// ```
    #[must_use]
    pub fn append(&self, additional: &Self) -> Self {
        Self::from(format!("{}\n{}", self.comment, additional.comment))
    }
}

impl<'a> From<Cow<'a, str>> for Comment<'a> {
    fn from(comment: Cow<'a, str>) -> Self {
        Self { comment }
    }
}

impl<'a> From<String> for Comment<'a> {
    fn from(comment: String) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}

impl<'a> From<&'a str> for Comment<'a> {
    fn from(comment: &'a str) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}

impl<'a> From<Comment<'a>> for String {
    fn from(comment: Comment<'a>) -> Self {
        comment.comment.into()
    }
}
