use crate::{Body, Comment};

/// A `Fragment` from the [`CommitMessage`], either a comment or body
#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
    Body(Body),
    Comment(Comment),
}
