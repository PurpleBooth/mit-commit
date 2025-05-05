use std::borrow::Cow;

use super::Subject;
use crate::{Comment, body::Body, fragment::Fragment};

#[test]
fn len() {
    assert_eq!(Subject::from("hello, world!").len(), 13);
    assert_eq!(Subject::from("goodbye").len(), 7);
}

#[test]
fn chars() {
    let subject = Subject::from("y\u{306}");

    let mut chars = subject.chars();

    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('\u{0306}'), chars.next());

    assert_eq!(None, chars.next());
}

#[test]
fn is_empty() {
    assert!(!Subject::from("hello, world!").is_empty());
    assert!(Subject::from("").is_empty());
}

#[test]
fn it_can_be_formatted() {
    let _subject = String::from(Subject::from("hello, world!"));

    assert_eq!(
        format!("{}", Subject::from("hello, world!")),
        String::from("hello, world!")
    );
}

#[test]
fn it_can_be_created_from_a_str() {
    let subject = String::from(Subject::from("hello, world!"));

    assert_eq!(subject, String::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_a_string() {
    let subject = String::from(Subject::from(String::from("hello, world!")));

    assert_eq!(subject, String::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_a_body() {
    let subject = Subject::from(Body::from("hello, world!"));

    assert_eq!(subject, Subject::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_fragments() {
    let subject = Subject::from(vec![Fragment::Body(Body::from("hello, world!"))]);

    assert_eq!(subject, Subject::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_a_cow() {
    let subject = Subject::from(Cow::from("hello, world!"));

    assert_eq!(subject, Subject::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_fragments_commit_first_is_skipped() {
    let subject = Subject::from(vec![
        Fragment::Comment(Comment::from("# Important Comment")),
        Fragment::Body(Body::from("hello, world!")),
    ]);

    assert_eq!(subject, Subject::from("hello, world!"));
}
