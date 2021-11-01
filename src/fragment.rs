use crate::{Body, Comment};

/// A `Fragment` from the [`CommitMessage`], either a comment or body
#[derive(Clone, Debug, PartialEq)]
pub enum Fragment<'a> {
    /// A fragment that is going to appear in the git log
    Body(Body<'a>),
    /// A fragment that is a comment
    Comment(Comment<'a>),
}

impl<'a> From<Body<'a>> for Fragment<'a> {
    fn from(body: Body<'a>) -> Self {
        Self::Body(body)
    }
}

impl<'a> From<Comment<'a>> for Fragment<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}
