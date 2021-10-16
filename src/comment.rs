/// A single comment from a `CommitMessage`
#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
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

impl From<String> for Comment {
    fn from(comment: String) -> Self {
        Self { comment }
    }
}

impl From<&str> for Comment {
    fn from(comment: &str) -> Self {
        Self {
            comment: String::from(comment),
        }
    }
}

impl From<Comment> for String {
    fn from(comment: Comment) -> Self {
        comment.comment
    }
}
