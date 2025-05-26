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
    /// # Returns
    ///
    /// An iterator over the comments in this collection
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
    /// # Returns
    ///
    /// An iterator that takes ownership of the comments
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
    /// # Returns
    ///
    /// An iterator over references to the comments
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
    /// Create Comments from a vector of Comment
    ///
    /// # Arguments
    ///
    /// * `comments` - The vector of comments to create the collection from
    ///
    /// # Returns
    ///
    /// A new Comments collection containing the provided comments
    fn from(comments: Vec<Comment<'a>>) -> Self {
        Self { comments }
    }
}

impl From<Comments<'_>> for String {
    /// Convert Comments to a String
    ///
    /// # Arguments
    ///
    /// * `comments` - The comments collection to convert
    ///
    /// # Returns
    ///
    /// A String containing all comments joined with double newlines
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
    /// Create Comments from a vector of Fragment
    ///
    /// # Arguments
    ///
    /// * `ast` - The vector of fragments to filter for comments
    ///
    /// # Returns
    ///
    /// A new Comments collection containing only the Comment fragments
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

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::body::Body;

    #[test]
    fn test_iterator_implementation() {
        let comments = Comments::from(vec![
            Comment::from("# Comment 1"),
            Comment::from("# Comment 2"),
            Comment::from("# Comment 3"),
        ]);
        let mut iterator = comments.iter();

        assert_eq!(
            iterator.next(),
            Some(&Comment::from("# Comment 1")),
            "Iterator should return the first comment"
        );
        assert_eq!(
            iterator.next(),
            Some(&Comment::from("# Comment 2")),
            "Iterator should return the second comment"
        );
        assert_eq!(
            iterator.next(),
            Some(&Comment::from("# Comment 3")),
            "Iterator should return the third comment"
        );
        assert_eq!(
            iterator.next(),
            None,
            "Iterator should return None after all comments"
        );
    }

    #[test]
    fn test_string_conversion() {
        let comments = Comments::from(vec![
            Comment::from("# Message Body"),
            Comment::from("# Another Message Body"),
        ]);

        assert_eq!(
            String::from(comments),
            String::from(indoc!(
                "
                # Message Body

                # Another Message Body"
            )),
            "Comments should convert to a string with comments separated by double newlines"
        );
    }

    #[test]
    fn test_creation_from_fragments() {
        let comments = Comments::from(vec![
            Fragment::Comment(Comment::from("# Message Body")),
            Fragment::Body(Body::from("Some body content")),
            Fragment::Comment(Comment::from("# Another Message Body")),
        ]);

        assert_eq!(
            comments,
            Comments::from(vec![
                Comment::from("# Message Body"),
                Comment::from("# Another Message Body"),
            ]),
            "Comments should be created from fragments, filtering out non-comment fragments"
        );
    }
}
