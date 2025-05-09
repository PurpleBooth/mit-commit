use std::borrow::Cow;

const LEGAL_CHARACTERS: [char; 10] = ['#', ';', '@', '!', '$', '%', '^', '&', '|', ':'];

/// A single comment from a `CommitMessage`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment<'a> {
    comment: Cow<'a, str>,
}

impl Comment<'_> {
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

    /// Tells you if a given comment character is a potential comment character
    ///
    /// # Example
    ///
    /// ```
    /// use mit_commit::Comment;
    ///
    /// assert!(!Comment::is_legal_comment_char('?'),);
    /// assert!(Comment::is_legal_comment_char('#'),);
    /// ```
    #[must_use]
    pub fn is_legal_comment_char(character: char) -> bool {
        LEGAL_CHARACTERS.contains(&character)
    }
}

impl<'a> From<Cow<'a, str>> for Comment<'a> {
    fn from(comment: Cow<'a, str>) -> Self {
        Self { comment }
    }
}

impl From<String> for Comment<'_> {
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
