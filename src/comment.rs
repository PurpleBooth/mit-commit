use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take_until1},
    Compare,
    InputLength,
    InputTake,
    sequence::pair,
};

pub(crate) const LEGAL_CHARACTERS: [char; 10] = ['#', ';', '@', '!', '$', '%', '^', '&', '|', ':'];
pub(crate) const LEGAL_CHARACTERS_STR: [&'static str; 10] =
    ["#", ";", "@", "!", "$", "%", "^", "&", "|", ":"];

/// A single comment from a `CommitMessage`
#[derive(Debug, PartialEq, Clone)]
pub struct Comment<'a> {
    comment: Cow<'a, str>,
}

impl<'a> Comment<'a> {
    pub fn parser<I, T, E: nom::error::ParseError<I>>(
        comment_character: T,
    ) -> impl FnMut(I) -> Result<(I, (I, I)), nom::Err<E>>
        where
            I: InputTake + Compare<T> + nom::FindSubstring<&'static str> + nom::InputIter,
            T: InputLength + Clone,
    {
        pair(tag(comment_character), take_until1("\n"))
    }

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
