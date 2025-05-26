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
    /// # Arguments
    ///
    /// * `additional` - The comment to append to this one
    ///
    /// # Returns
    ///
    /// A new comment with the content of both comments separated by a newline
    ///
    /// # Examples
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

    /// Checks if a given character is a valid comment character
    ///
    /// # Arguments
    ///
    /// * `character` - The character to check
    ///
    /// # Returns
    ///
    /// `true` if the character is a valid comment character, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Comment;
    ///
    /// assert!(!Comment::is_legal_comment_char('?'));
    /// assert!(Comment::is_legal_comment_char('#'));
    /// ```
    #[must_use]
    pub fn is_legal_comment_char(character: char) -> bool {
        LEGAL_CHARACTERS.contains(&character)
    }
}

impl<'a> From<Cow<'a, str>> for Comment<'a> {
    /// Create a Comment from a Cow<_, str>
    ///
    /// # Arguments
    ///
    /// * `comment` - The string content to create the comment from
    ///
    /// # Returns
    ///
    /// A new Comment containing the provided string
    fn from(comment: Cow<'a, str>) -> Self {
        Self { comment }
    }
}

impl From<String> for Comment<'_> {
    /// Create a Comment from a String
    ///
    /// # Arguments
    ///
    /// * `comment` - The string to create the comment from
    ///
    /// # Returns
    ///
    /// A new Comment containing the provided string
    fn from(comment: String) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}

impl<'a> From<&'a str> for Comment<'a> {
    /// Create a Comment from a string slice
    ///
    /// # Arguments
    ///
    /// * `comment` - The string slice to create the comment from
    ///
    /// # Returns
    ///
    /// A new Comment containing the provided string
    fn from(comment: &'a str) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}

impl<'a> From<Comment<'a>> for String {
    /// Convert a Comment to a String
    ///
    /// # Arguments
    ///
    /// * `comment` - The comment to convert
    ///
    /// # Returns
    ///
    /// A String containing the comment's text
    fn from(comment: Comment<'a>) -> Self {
        comment.comment.into()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_creation_from_str() {
        let comment = Comment::from("# Example Comment");

        assert_eq!(
            String::from(comment),
            String::from("# Example Comment"),
            "Comment should convert to the correct string when created from a str"
        );
    }

    #[test]
    fn test_creation_from_string() {
        let comment = Comment::from(String::from("# Example Comment"));

        assert_eq!(
            String::from(comment),
            String::from("# Example Comment"),
            "Comment should convert to the correct string when created from a String"
        );
    }

    #[test]
    fn test_legal_comment_char_detection() {
        assert!(
            Comment::is_legal_comment_char('#'),
            "# should be recognized as a legal comment character"
        );
    }

    #[test]
    fn test_illegal_comment_char_detection() {
        assert!(
            !Comment::is_legal_comment_char('?'),
            "? should not be recognized as a legal comment character"
        );
    }

    #[test]
    fn test_append_comment_fragments() {
        assert_eq!(
            Comment::from(indoc!(
                "
                Example 1
                Example 2"
            )),
            Comment::from("Example 1").append(&Comment::from("Example 2")),
            "Appending comments should create a new comment with content separated by newline"
        );
    }
}
