use crate::comment::Comment;
use crate::fragment::Fragment;

/// A collection of comments from a `CommitMessage`
#[derive(Debug, PartialEq, Clone)]
pub struct Comments {
    comments: Vec<Comment>,
}

impl From<Vec<Comment>> for Comments {
    fn from(comments: Vec<Comment>) -> Self {
        Comments { comments }
    }
}

impl From<Comments> for String {
    fn from(comments: Comments) -> Self {
        comments
            .comments
            .into_iter()
            .map(String::from)
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

#[cfg(test)]
mod tests {
    use super::Comments;
    use crate::body::Body;
    use crate::comment::Comment;
    use crate::fragment::Fragment;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_give_me_it_as_a_string() {
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
            ))
        )
    }

    #[test]
    fn it_can_create_itself_from_an_ast() {
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
            ])
        )
    }
}
