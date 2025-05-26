use crate::{Body, Comment};

/// A `Fragment` from the [`CommitMessage`], either a comment or body
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fragment<'a> {
    /// A fragment that is going to appear in the git log
    Body(Body<'a>),
    /// A fragment that is a comment
    Comment(Comment<'a>),
}

impl<'a> From<Body<'a>> for Fragment<'a> {
    /// Create a Fragment from a Body
    ///
    /// # Arguments
    ///
    /// * `body` - The body to convert into a fragment
    ///
    /// # Returns
    ///
    /// A new `Fragment::Body` variant containing the provided body
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Body, Fragment};
    ///
    /// let body = Body::from("Example body");
    /// let fragment = Fragment::from(body.clone());
    /// assert_eq!(fragment, Fragment::Body(body));
    /// ```
    fn from(body: Body<'a>) -> Self {
        Self::Body(body)
    }
}

impl<'a> From<Comment<'a>> for Fragment<'a> {
    /// Create a Fragment from a Comment
    ///
    /// # Arguments
    ///
    /// * `comment` - The comment to convert into a fragment
    ///
    /// # Returns
    ///
    /// A new `Fragment::Comment` variant containing the provided comment
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Comment, Fragment};
    ///
    /// let comment = Comment::from("# Example comment");
    /// let fragment = Fragment::from(comment.clone());
    /// assert_eq!(fragment, Fragment::Comment(comment));
    /// ```
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_conversion_to_fragment() {
        let body: Body<'_> = "A Body".into();
        let fragment: Fragment<'_> = body.clone().into();

        assert_eq!(
            fragment,
            Fragment::Body(body),
            "Converting a Body to a Fragment should create a Fragment::Body variant with the same content"
        );
    }

    #[test]
    fn test_comment_conversion_to_fragment() {
        let comment: Comment<'_> = "A Comment".into();
        let fragment: Fragment<'_> = comment.clone().into();

        assert_eq!(
            fragment,
            Fragment::Comment(comment),
            "Converting a Comment to a Fragment should create a Fragment::Comment variant with the same content"
        );
    }
}
