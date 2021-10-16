use indoc::indoc;

use super::Body;

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
