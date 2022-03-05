use std::{convert::TryInto, io::Write};

use indoc::indoc;
use quickcheck::TestResult;
use regex::Regex;
use tempfile::NamedTempFile;

use super::CommitMessage;
use crate::{
    bodies::Bodies,
    body::Body,
    comment::Comment,
    scissors::Scissors,
    subject::Subject,
    trailer::Trailer,
    Fragment,
};

#[test]
fn implements_default() {
    let commit = CommitMessage::default();
    let actual: String = commit.into();

    assert_eq!(actual, String::new());
}

#[test]
fn can_check_if_it_matches_pattern() {
    let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153
            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    let re = Regex::new("[Bb]itte").unwrap();
    assert!(!commit.matches_pattern(&re));

    let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
    assert!(commit.matches_pattern(&re));

    let re = Regex::new("[Ee]xample Commit Message").unwrap();
    assert!(commit.matches_pattern(&re));

    let re = Regex::new("Relates[- ]to").unwrap();
    assert!(commit.matches_pattern(&re));
}

#[test]
fn can_parse_when_there_is_no_gutter() {
    let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message
            This is an example commit message for linting

            This is another line
            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    assert_eq!(
        commit.get_subject(),
        Subject::from("Example Commit Message\nThis is an example commit message for linting")
    );
    assert_eq!(
        commit.get_body(),
        Bodies::from(vec![Body::default(), Body::from("This is another line")])
    );
}

#[test]
fn can_add_trailers_to_a_normal_commit() {
    let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    assert_eq!(
        String::from(commit.add_trailer(Trailer::new("Co-authored-by".into(), "Test Trailer <test@example.com>".into()))),
        String::from(CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153
            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ))));
}

#[test]
fn can_add_trailers_to_a_commit_without_existing_trailers() {
    let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    let expected = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
    assert_eq!(
        String::from(commit.add_trailer(Trailer::new(
            "Co-authored-by".into(),
            "Test Trailer <test@example.com>".into(),
        ))),
        String::from(expected)
    );
}

#[test]
fn can_add_trailers_to_an_empty_commit() {
    let commit = CommitMessage::from(indoc!(
            "

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    let expected = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
    assert_eq!(
        String::from(commit.add_trailer(Trailer::new(
            "Co-authored-by".into(),
            "Test Trailer <test@example.com>".into(),
        ))),
        String::from(expected)
    );
}

#[test]
fn can_add_trailers_to_an_empty_commit_with_single_trailer() {
    let commit = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

    let expected = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>
            Co-authored-by: Someone Else <someone@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
    assert_eq!(
        String::from(commit.add_trailer(Trailer::new(
            "Co-authored-by".into(),
            "Someone Else <someone@example.com>".into(),
        ))),
        String::from(expected)
    );
}

#[test]
fn can_generate_a_commit_from_an_ast() {
    let message = CommitMessage::from_fragments(
        vec![
            Fragment::Body(Body::from("Example Commit")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Here is a body")),
            Fragment::Comment(Comment::from("# Example Commit")),
        ],
        Some(Scissors::from(indoc!(
            "
            # ------------------------ >8 ------------------------
            # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            new file mode 100644
            index 0000000..e69de29
            "
        ))),
    );

    assert_eq!(
        String::from(message),
        String::from(indoc!(
            "
            Example Commit

            Here is a body
            # Example Commit
            # ------------------------ >8 ------------------------
            # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            new file mode 100644
            index 0000000..e69de29
            "
        ))
    );
}

#[test]
fn insert_after_last_body() {
    let ast: Vec<Fragment<'_>> = vec![
        Fragment::Body(Body::from("Add file")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Relates-to: #128")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
    ];
    let commit = CommitMessage::from_fragments(ast, None);

    assert_eq!(commit.insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))]).get_ast(), vec![
        Fragment::Body(Body::from("Add file")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Relates-to: #128\nRelates-to: #656")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
    ]);
}

#[test]
fn insert_after_last_body_with_no_body() {
    let ast: Vec<Fragment<'_>> = vec![
        Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
    ];
    let commit = CommitMessage::from_fragments(ast, None);

    assert_eq!(commit.insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))]).get_ast(), vec![
        Fragment::Body(Body::from("Relates-to: #656")),
        Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
    ]);
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn with_subject(input: String) -> bool {
    let commit: CommitMessage<'_> = "Some Subject".into();
    let actual: String = commit
        .with_subject(input.clone().into())
        .get_subject()
        .into();
    actual == input
}

#[test]
fn with_subject_on_default_commit() {
    let commit = CommitMessage::default().with_subject("Subject".into());
    assert_eq!(commit.get_subject(), Subject::from("Subject"));
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn with_body(input: String) -> TestResult {
    if input.contains('\r') {
        return TestResult::discard();
    }

    let commit: CommitMessage<'_> = "Some Subject\n\nSome Body".into();
    let expected: String = format!("Some Subject\n\n{}", input);
    let actual: String = commit.with_body_contents(&input).into();
    TestResult::from_bool(actual == expected)
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn with_body_with_no_gutter(input: String) -> TestResult {
    if input.contains('\r') {
        return TestResult::discard();
    }

    let commit: CommitMessage<'_> = "Some Subject\nSome More Subject\n\nBody".into();
    let expected: String = format!("Some Subject\nSome More Subject\n\n{}", input);
    let actual: String = commit.with_body_contents(&input).into();
    TestResult::from_bool(actual == expected)
}

#[test]
fn can_get_comment_character_when_there_is_no_comments() {
    let commit_character = CommitMessage::from("Example Commit Message");
    assert!(commit_character.get_comment_char().is_none());
}

#[test]
fn can_read_from_path_buf() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(temp_file.as_file(), "Some Subject").expect("Failed to write file");

    let commit_character: CommitMessage<'_> = temp_file
        .path()
        .to_path_buf()
        .try_into()
        .expect("Could not read commit message");
    assert_eq!(commit_character.get_subject().to_string(), "Some Subject");
}

#[test]
fn can_read_from_path() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(temp_file.as_file(), "Some Subject").expect("Failed to write file");

    let commit_character: CommitMessage<'_> = temp_file
        .path()
        .try_into()
        .expect("Could not read commit message");
    assert_eq!(commit_character.get_subject().to_string(), "Some Subject");
}
