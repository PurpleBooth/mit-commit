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

#[test]
fn parser_without_a_comment_character_fails() {
    let mut parser = Comment::parser('#');
    assert!(parser("Example Body")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .is_err())
}

#[test]
fn parser_with_a_comment_character_and_without_a_newline() {
    let mut parser = Comment::parser('#');
    let actual = parser("# Example Comment")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(actual, ("", Comment::from("# Example Comment")))
}

#[test]
fn parser_can_be_given_a_different_comment_char() {
    let mut parser = Comment::parser(';');
    let actual = parser("; Example Comment")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(actual, ("", Comment::from("; Example Comment")))
}

#[test]
fn parser_with_a_comment_character_and_with_a_newline() {
    let mut parser = Comment::parser('#');
    let actual = parser("# Example Comment\nSome body")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(actual, ("Some body", Comment::from("# Example Comment\n")))
}
