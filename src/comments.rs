use std::slice::Iter;

use crate::{comment::Comment, fragment::Fragment};

/// A collection of comments from a [`CommitMessage`]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Comments {
    comments: Vec<Comment>,
}

impl Comments {
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
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Comment> {
        self.comments.iter()
    }
}

impl IntoIterator for Comments {
    type IntoIter = std::vec::IntoIter<Comment>;
    type Item = Comment;

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

impl From<Vec<Comment>> for Comments {
    fn from(comments: Vec<Comment>) -> Self {
        Self { comments }
    }
}

impl From<Comments> for String {
    fn from(comments: Comments) -> Self {
        comments
            .comments
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl From<Vec<Fragment>> for Comments {
    fn from(ast: Vec<Fragment>) -> Self {
        ast.iter()
            .filter_map(|values| {
                if let Fragment::Comment(comment) = values {
                    Some(comment.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Comment>>()
            .into()
    }
}
