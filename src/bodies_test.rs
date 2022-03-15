use indoc::indoc;

use crate::{body::Body, fragment::Fragment};

use super::Bodies;

#[test]
fn implements_iterator() {
    let trailers = Bodies::from(vec![
        Body::from("Body 1"),
        Body::from("Body 2"),
        Body::from("Body 3"),
    ]);
    let mut iterator = trailers.iter();

    assert_eq!(iterator.next(), Some(&Body::from("Body 1")));
    assert_eq!(iterator.next(), Some(&Body::from("Body 2")));
    assert_eq!(iterator.next(), Some(&Body::from("Body 3")));
    assert_eq!(iterator.next(), None);
}

#[test]
fn it_can_give_me_it_as_a_string() {
    let bodies = Bodies::from(vec![
        Body::from("Message Body"),
        Body::from("Another Message Body"),
    ]);

    assert_eq!(
        String::from(bodies),
        String::from(indoc!(
            "
            Message Body

            Another Message Body"
        ))
    );
}

#[test]
fn it_can_be_formatted() {
    let bodies = Bodies::from(vec![
        Body::from("Message Body"),
        Body::from("Another Message Body"),
    ]);

    assert_eq!(
        format!("{}", bodies),
        String::from(indoc!(
            "
            Message Body

            Another Message Body"
        ))
    );
}

#[test]
fn get_first() {
    let bodies = Bodies::from(vec![
        Body::from("Message Body"),
        Body::from("Another Message Body"),
    ]);

    assert_eq!(bodies.first(), Some(Body::from("Message Body")));
}

#[test]
fn it_can_parse_itself_from_an_ast() {
    let bodies = Bodies::from(vec![
        Fragment::Body(Body::from("Subject Line")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Some content in the body of the message")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from(indoc!(
            "
            Co-authored-by: Billie Thomposon <billie@example.com>
            Co-authored-by: Someone Else <someone@example.com>
            "
        ))),
    ]);

    assert_eq!(
        bodies,
        Bodies::from(vec![
            Body::default(),
            Body::from("Some content in the body of the message"),
        ])
    );
}
