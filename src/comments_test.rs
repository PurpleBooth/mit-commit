use indoc::indoc;

use super::Comments;
use crate::{body::Body, comment::Comment, fragment::Fragment};

#[test]
fn implements_iterator() {
    use crate::{Comment, Comments};
    let trailers = Comments::from(vec![
        Comment::from("# Comment 1"),
        Comment::from("# Comment 2"),
        Comment::from("# Comment 3"),
    ]);
    let mut iterator = trailers.iter();

    assert_eq!(iterator.next(), Some(&Comment::from("# Comment 1")));
    assert_eq!(iterator.next(), Some(&Comment::from("# Comment 2")));
    assert_eq!(iterator.next(), Some(&Comment::from("# Comment 3")));
    assert_eq!(iterator.next(), None);
}

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
    );
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
    );
}
