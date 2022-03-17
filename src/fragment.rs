use nom::{branch::alt, combinator::map, IResult};

use crate::{Body, Comment};

/// A `Fragment` from the [`CommitMessage`], either a comment or body
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fragment<'a> {
    /// A fragment that is going to appear in the git log
    Body(Body<'a>),
    /// A fragment that is a comment
    Comment(Comment<'a>),
}

impl<'a> Fragment<'a> {
    /// Build a parser for both body and comment fragments
    pub fn parser<E: nom::error::ParseError<&'a str> + 'a>(
        comment_char: char,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Fragment<'a>, E> + 'a
    where
        E: 'a,
    {
        return alt((
            map(Comment::parser(comment_char), Fragment::Comment),
            map(Body::parser(comment_char), Fragment::Body),
        ));
    }
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
