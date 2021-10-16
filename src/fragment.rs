use crate::{Body, Comment};

/// A `Fragment` from the [`CommitMessage`], either a comment or body
#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
    Body(Body),
    Comment(Comment),
}

impl From<Body> for Fragment {
    fn from(body: Body) -> Self {
        Self::Body(body)
    }
}

impl From<&Body> for Fragment {
    fn from(body: &Body) -> Self {
        body.clone().into()
    }
}

impl From<Comment> for Fragment {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}

impl From<&Comment> for Fragment {
    fn from(comment: &Comment) -> Self {
        comment.clone().into()
    }
}
