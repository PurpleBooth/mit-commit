use nom::multi::many0;

use crate::{Body, Comment, Fragment};

#[test]
fn can_convert_body_into_a_fragment() {
    let body: Body<'_> = "A Body".into();
    let fragment: Fragment<'_> = body.clone().into();

    assert_eq!(fragment, Fragment::Body(body));
}

#[test]
fn can_convert_comment_into_a_fragment() {
    let comment: Comment<'_> = "A Comment".into();
    let fragment: Fragment<'_> = comment.clone().into();

    assert_eq!(fragment, Fragment::Comment(comment));
}

#[test]
fn can_parse_a_body_into_a_fragment() {
    let mut parser = Fragment::parser('#');

    assert_eq!(
        parser("Hello, world!")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        ("", Fragment::Body("Hello, world!".into()))
    );
}

#[test]
fn can_parse_a_comment_into_a_fragment() {
    let mut parser = Fragment::parser('#');

    assert_eq!(
        parser("# Hello, world!")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        ("", Fragment::Comment("# Hello, world!".into()))
    );
}

#[test]
fn can_pull_out_a_list() {
    let mut parser = many0(Fragment::parser('#'));

    assert_eq!(
        parser("# Hello, world!\nGoodbye\n, world!\n\nExample Text")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        (
            "",
            vec![
                Fragment::Comment("# Hello, world!\n".into()),
                Fragment::Body("Goodbye\n, world!\n\n".into()),
                Fragment::Body("Example Text".into())
            ]
        )
    );
}

#[test]
fn one_comment_per_line() {
    let mut parser = Fragment::parser('#');

    assert_eq!(
        parser("# Hello, world!\n# Another Comment\nAnother Body")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        (
            "# Another Comment\nAnother Body",
            Fragment::Comment("# Hello, world!\n".into())
        )
    );
}

#[test]
fn bodies_are_bundled_together_when_terminating_with_a_double_newline() {
    let mut parser = Fragment::parser('#');

    assert_eq!(
        parser("A Body\nAnother Body\n\n")
            .map_err(|err: nom::Err<nom::error::Error<&str>>| err.to_owned())
            .unwrap(),
        ("", Fragment::Body("A Body\nAnother Body\n\n".into()))
    );
}
