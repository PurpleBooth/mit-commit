use std::slice::Iter;

use crate::{comment::Comment, fragment::Fragment};

/// A collection of comments from a [`CommitMessage`]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Comments<'a> {
    comments: Vec<Comment<'a>>,
}

impl Comments<'_> {
    /// Iterate over the [`Comment`] in the [`Comments`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Comment, Comments};
    ///
    /// let trailers = Comments::from(vec![
    ///     Comment::from("# Comment 1"),
    ///     Comment::from("# Comment 2"),
    ///     Comment::from("# Comment 3"),
    /// ]);
    /// let mut iterator = trailers.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 1")));
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 2")));
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, Comment<'_>> {
        self.comments.iter()
    }
}

impl<'a> IntoIterator for Comments<'a> {
    type IntoIter = std::vec::IntoIter<Comment<'a>>;
    type Item = Comment<'a>;

    /// Iterate over the [`Comment`] in the [`Comments`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Comment, Comments};
    ///
    /// let trailers = Comments::from(vec![
    ///     Comment::from("# Comment 1"),
    ///     Comment::from("# Comment 2"),
    ///     Comment::from("# Comment 3"),
    /// ]);
    /// let mut iterator = trailers.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(Comment::from("# Comment 1")));
    /// assert_eq!(iterator.next(), Some(Comment::from("# Comment 2")));
    /// assert_eq!(iterator.next(), Some(Comment::from("# Comment 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.comments.into_iter()
    }
}

impl<'a> IntoIterator for &'a Comments<'a> {
    type IntoIter = Iter<'a, Comment<'a>>;
    type Item = &'a Comment<'a>;

    /// Iterate over the [`Comment`] in the [`Comments`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Borrow;
    ///
    /// use mit_commit::{Comment, Comments};
    ///
    /// let comments = Comments::from(vec![
    ///     Comment::from("# Comment 1"),
    ///     Comment::from("# Comment 2"),
    ///     Comment::from("# Comment 3"),
    /// ]);
    /// let comments_ref = comments.borrow();
    /// let mut iterator = comments_ref.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 1")));
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 2")));
    /// assert_eq!(iterator.next(), Some(&Comment::from("# Comment 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.comments.iter()
    }
}

impl<'a> From<Vec<Comment<'a>>> for Comments<'a> {
    fn from(comments: Vec<Comment<'a>>) -> Self {
        Self { comments }
    }
}

impl From<Comments<'_>> for String {
    fn from(comments: Comments<'_>) -> Self {
        comments
            .comments
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl<'a> From<Vec<Fragment<'a>>> for Comments<'a> {
    fn from(ast: Vec<Fragment<'a>>) -> Self {
        ast.into_iter()
            .filter_map(|values| {
                if let Fragment::Comment(comment) = values {
                    Some(comment)
                } else {
                    None
                }
            })
            .collect::<Vec<Comment<'_>>>()
            .into()
    }
}
