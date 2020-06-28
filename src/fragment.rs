use crate::{Body, Comment};

#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
    Body(Body),
    Comment(Comment),
}
