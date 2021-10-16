use indoc::indoc;

use super::Trailers;
use crate::{fragment::Fragment, trailer::Trailer, Body};

#[test]
fn implements_iterator() {
    let trailers = Trailers::from(vec![
        Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
        Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
        Trailer::new("Relates-to", "#124"),
    ]);
    let mut iterator = trailers.iter();

    assert_eq!(
        iterator.next(),
        Some(&Trailer::new(
            "Co-authored-by",
            "Billie Thompson <billie@example.com>",
        ))
    );
    assert_eq!(
        iterator.next(),
        Some(&Trailer::new(
            "Co-authored-by",
            "Someone Else <someone@example.com>",
        ))
    );
    assert_eq!(iterator.next(), Some(&Trailer::new("Relates-to", "#124")));
    assert_eq!(iterator.next(), None);
}

#[test]
fn it_can_give_me_it_as_a_string() {
    let trailers = Trailers::from(vec![Trailer::new(
        "Co-authored-by",
        "Billie Thompson <billie@example.com>",
    )]);

    assert_eq!(
        String::from(trailers),
        String::from("Co-authored-by: Billie Thompson <billie@example.com>")
    );
}

#[test]
fn it_can_give_me_the_length() {
    let trailers = Trailers::from(vec![
        Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
        Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    ]);

    assert_eq!(trailers.len(), 2);
}

#[test]
fn it_can_tell_me_if_it_is_empty() {
    assert!(!Trailers::from(vec![
        Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
        Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    ])
    .is_empty());

    let trailers: Vec<Trailer> = Vec::new();
    assert!(Trailers::from(trailers).is_empty());
}

#[test]
fn it_can_be_constructed_from_ast() {
    let trailers = vec![
        Fragment::Body(Body::from("Example Commit")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from(indoc!(
            "
                This is an example commit. This is to illustrate something for a test and would be
                pretty unusual to find in an actual git history.
                "
        ))),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from(
            "Co-authored-by: Billie Thompson <billie@example.com>",
        )),
        Fragment::Body(Body::from(
            "Co-authored-by: Somebody Else <somebody@example.com>",
        )),
    ];

    let expected: Trailers = vec![
        Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
        Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
    ]
    .into();

    assert_eq!(Trailers::from(trailers), expected);
}
