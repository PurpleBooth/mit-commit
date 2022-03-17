use super::Subject;
use crate::{body::Body, fragment::Fragment, Comment};

#[test]
fn can_read_subjects() {
    let parsed: (&str, Subject<'_>) =
        Subject::parser::<nom::error::Error<_>>('#')("Subject\n\nBody\n# Comment".into())
            .expect("Parsing was not successful");

    assert_eq!(parsed, ("Body\n# Comment", Subject::from("Subject\n\n"),));
}

#[test]
fn can_read_multiline_subjects() {
    let parsed: (&str, Subject<'_>) =
        Subject::parser::<nom::error::Error<_>>('#')("Subject\nSecond Bit\n\nBody".into())
            .expect("Parsing was not successful");

    assert_eq!(parsed, ("Body", Subject::from("Subject\nSecond Bit\n\n"),));
}

#[test]
fn skips_comments() {
    let parsed: (&str, Subject<'_>) =
        Subject::parser::<nom::error::Error<_>>('#')("# Comment\nSubject\n\nBody".into())
            .expect("Parsing was not successful");

    assert_eq!(parsed, ("Body", Subject::from("Subject\n\n"),));
}

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
fn it_can_be_created_from_fragments_commit_first_is_skipped() {
    let subject = Subject::from(vec![
        Fragment::Comment(Comment::from("# Important Comment")),
        Fragment::Body(Body::from("hello, world!")),
    ]);

    assert_eq!(subject, Subject::from("hello, world!"));
}
