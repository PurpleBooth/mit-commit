use indoc::indoc;

use super::Comment;

#[test]
fn it_can_be_created_from_a_str() {
    let comment = Comment::from("# Example Comment");

    assert_eq!(String::from(comment), String::from("# Example Comment"));
}

#[test]
fn it_can_be_created_from_a_string() {
    let comment = Comment::from(String::from("# Example Comment"));

    assert_eq!(String::from(comment), String::from("# Example Comment"));
}
#[test]
fn it_can_tell_me_if_a_char_is_a_legal_comment_char() {
    assert!(Comment::is_legal_comment_char('#'));
}

#[test]
fn it_can_tell_me_if_a_char_is_not_legal_comment_char() {
    assert!(!Comment::is_legal_comment_char('?'));
}

#[test]
fn it_can_append_another_comment_fragment() {
    assert_eq!(
        Comment::from(indoc!(
            "
            Example 1
            Example 2"
        )),
        Comment::from("Example 1").append(&Comment::from("Example 2"))
    );
}
