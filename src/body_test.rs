use indoc::indoc;

use super::Body;

#[test]
fn it_can_parse_a_body_without_newlines() {
    let mut parser = Body::parser('#');
    let actual = parser("Example Body")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(actual, ("", Body::from("Example Body")))
}

#[test]
fn it_can_parse_a_body_with_paragraph_breaks() {
    let mut parser = Body::parser('#');
    let actual = parser("Example Body\nSomething else\n\nMore")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(
        actual,
        ("More", Body::from("Example Body\nSomething else\n\n"))
    );
}

#[test]
fn it_does_not_get_confused_by_comments() {
    let mut parser = Body::parser('#');
    let actual = parser("Example Body\n# Something else\n\nMore")
        .map_err(|x: nom::Err<nom::error::Error<&str>>| x.to_owned())
        .unwrap();

    assert_eq!(
        actual,
        ("# Something else\n\nMore", Body::from("Example Body\n"))
    );
}

#[test]
fn bodies_are_bundled_together_when_terminating_with_a_single_newline() {
    let mut parser = Body::parser('#');

    assert_eq!(
        parser("A Body\nAnother Body\n")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        ("", Body::from("A Body\nAnother Body\n"))
    );
}

#[test]
fn bodies_are_bundled_together_when_terminating_without_a_newline() {
    let mut parser = Body::parser('#');

    assert_eq!(
        parser("A Body\nAnother Body")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        ("", Body::from("A Body\nAnother Body"))
    );
}

#[test]
fn it_can_give_me_it_as_a_string_from_a_str() {
    let body = Body::from("Example Body");

    assert_eq!(String::from(body), String::from("Example Body"));
}

#[test]
fn it_can_give_me_it_as_a_string_from_a_string() {
    let body = Body::from(String::from("Example Body"));

    assert_eq!(String::from(body), String::from("Example Body"));
}

#[test]
fn it_implements_display() {
    let body = Body::from("Example Body");

    assert_eq!(format!("{}", body), "Example Body");
}

#[test]
fn it_can_append_another_body_fragment() {
    assert_eq!(
        Body::from(indoc!(
            "
            Example 1
            Example 2"
        )),
        Body::from("Example 1").append(&Body::from("Example 2"))
    );
}

#[test]
fn it_can_tell_me_if_it_is_empty() {
    assert!(Body::from("").is_empty());
}

#[test]
fn it_can_tell_me_if_it_is_full() {
    assert!(!Body::from("something").is_empty());
}
