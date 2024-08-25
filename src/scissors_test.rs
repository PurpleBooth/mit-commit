use std::borrow::Cow;

use indoc::indoc;

use super::Scissors;

#[test]
fn can_give_me_it_as_string() {
    let message = String::from(Scissors::from("hello, world!"));

    assert_eq!(message, String::from("hello, world!"));
}

#[test]
fn it_can_be_created_from_a_string() {
    let message = String::from(Scissors::from(String::from("hello, world!")));

    assert_eq!(message, String::from("hello, world!"));
}

#[test]
fn it_can_guess_the_comment_character_from_scissors_with_required_space() {
    let comment_char = Scissors::guess_comment_character(indoc!(
        "
        Some text

        #------------------------ >8 ------------------------
        ; ------------------------ >8 ------------------------
        ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        ; Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ));

    assert_eq!(comment_char, Some(';'));
}

#[test]
fn it_can_guess_the_comment_character_from_scissors_without_comment() {
    let comment_char = Scissors::guess_comment_character(indoc!(
        "
        Some text

          ------------------------ >8 ------------------------
        ; ------------------------ >8 ------------------------
        ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        ; Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        "
    ));

    assert_eq!(comment_char, Some(';'));
}

#[test]
fn it_returns_none_on_a_failure_to_find_the_comment_char_from_scissors() {
    let comment_char = Scissors::guess_comment_character(indoc!(
        "
        Some text
        "
    ));

    assert_eq!(comment_char, None);
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
