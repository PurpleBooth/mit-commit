use std::borrow::Cow;

use indoc::indoc;

use super::Scissors;

#[test]
fn can_give_me_itself_as_string() {
    let message = String::from(Scissors::from("hello, world!"));

    assert_eq!(message, String::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_a_string() {
    let message = String::from(Scissors::from(String::from("hello, world!")));

    assert_eq!(message, String::from("hello, world!"));
}

#[test]
fn it_can_extract_itself_from_commit() {
    let sections = Scissors::parse_sections(indoc!(
        "
        Some text

        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ));

    assert_eq!(
        sections,
        (
            Cow::from("Some text\n"),
            Some(Scissors::from(indoc!(
                "
                # ------------------------ >8 ------------------------
                # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/file b/file
                "
            )))
        )
    );
}

#[test]
fn it_can_extract_itself_from_commit_with_a_standard_commit() {
    let sections = Scissors::parse_sections(indoc!(
        "
        Some text

        \u{00A3} ------------------------ >8 ------------------------
        \u{00A3} \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        \u{00A3} Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file"
    ));

    assert_eq!(
        sections,
        (
            Cow::from("Some text\n"),
            Some(Scissors::from(indoc!(
                "
                \u{00A3} ------------------------ >8 ------------------------
                \u{00A3} \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                \u{00A3} Alles unterhalb von ihr wird ignoriert.
                diff --git a/file b/file"
            )))
        )
    );
}

#[test]
fn parser_fails_with_non_scissors_content() {
    let mut parser = Scissors::parser('#');
    assert!(parser(indoc!(
        "
        Some text

        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ))
    .map_err(|error: nom::Err<nom::error::Error<&str>>| error.to_owned())
    .is_err());
}

#[test]
fn parser_can_take_the_scissors_section() {
    let mut parser = Scissors::parser('#');
    let result = parser(indoc!(
        "
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ))
    .map_err(|error: nom::Err<nom::error::Error<&str>>| error.to_owned())
    .unwrap();

    assert_eq!(
        result,
        (
            "",
            Scissors::from(indoc!(
                "
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
            ))
        )
    );
}

#[test]
fn parser_can_be_given_a_different_comment_character() {
    let mut parser = Scissors::parser(';');
    let result = parser(indoc!(
        "
        ; ------------------------ >8 ------------------------
        ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        ; Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ))
    .map_err(|error: nom::Err<nom::error::Error<&str>>| error.to_owned())
    .unwrap();

    assert_eq!(
        result,
        (
            "",
            Scissors::from(indoc!(
                "
        ; ------------------------ >8 ------------------------
        ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        ; Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
            ))
        )
    );
}
