use std::{convert::TryInto, io::Write};

use indoc::indoc;
use quickcheck::TestResult;
use regex::Regex;
use tempfile::NamedTempFile;

use super::CommitMessage;
use crate::{
    Fragment, bodies::Bodies, body::Body, comment::Comment, scissors::Scissors, subject::Subject,
    trailer::Trailer,
};

#[test]
fn test_default_returns_empty_string() {
    let commit = CommitMessage::default();
    let actual: String = commit.into();

    assert_eq!(
        actual,
        String::new(),
        "Default CommitMessage should convert to an empty string"
    );
}

#[test]
fn test_matches_pattern_returns_correct_results() {
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
    assert!(
        !commit.matches_pattern(&re),
        "Pattern should not match in comments"
    );

    let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
    assert!(
        commit.matches_pattern(&re),
        "Pattern should match in body text"
    );

    let re = Regex::new("[Ee]xample Commit Message").unwrap();
    assert!(
        commit.matches_pattern(&re),
        "Pattern should match in subject"
    );

    let re = Regex::new("Relates[- ]to").unwrap();
    assert!(
        commit.matches_pattern(&re),
        "Pattern should match in trailers"
    );
}

#[test]
fn test_parse_message_without_gutter_succeeds() {
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
        Subject::from("Example Commit Message\nThis is an example commit message for linting"),
        "Subject should include both lines when there's no gutter"
    );
    assert_eq!(
        commit.get_body(),
        Bodies::from(vec![Body::default(), Body::from("This is another line")]),
        "Body should contain the line after the empty line"
    );
}

#[test]
fn test_add_trailer_to_normal_commit_appends_correctly() {
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

    let expected = CommitMessage::from(indoc!(
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
    ));

    let actual = commit.add_trailer(Trailer::new(
        "Co-authored-by".into(),
        "Test Trailer <test@example.com>".into(),
    ));

    assert_eq!(
        String::from(actual),
        String::from(expected),
        "Adding a trailer to a commit with existing trailers should append the new trailer after the last trailer"
    );
}

#[test]
fn test_add_trailer_to_conventional_commit_appends_correctly() {
    let commit = CommitMessage::from(indoc!(
        "
        feat: Example Commit Message

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
        feat: Example Commit Message

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

    let actual = commit.add_trailer(Trailer::new(
        "Co-authored-by".into(),
        "Test Trailer <test@example.com>".into(),
    ));

    assert_eq!(
        String::from(actual),
        String::from(expected),
        "Adding a trailer to a conventional commit should append the trailer after the body"
    );
}

#[test]
fn test_add_trailer_to_commit_without_trailers_creates_trailer_section() {
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
        String::from(expected),
        "Adding a trailer to a commit without existing trailers should create a new trailer section after the body"
    );
}

#[test]
fn test_add_trailer_to_empty_commit_creates_trailer_section() {
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
        String::from(expected),
        "Adding a trailer to an empty commit should create a trailer section at the beginning"
    );
}

#[test]
fn test_add_trailer_to_empty_commit_with_trailer_appends_correctly() {
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
        String::from(expected),
        "Adding a trailer to an empty commit with an existing trailer should append the new trailer after the existing one"
    );
}

#[test]
fn test_from_fragments_generates_correct_commit() {
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
        )),
        "Creating a CommitMessage from fragments should generate the correct string representation"
    );
}

#[test]
fn test_insert_after_last_body_appends_correctly() {
    let ast: Vec<Fragment<'_>> = vec![
        Fragment::Body(Body::from("Add file")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from(
            "This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.",
        )),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Relates-to: #128")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from(
            "# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here",
        )),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from(
            "# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#",
        )),
    ];
    let commit = CommitMessage::from_fragments(ast, None);

    assert_eq!(
        commit
            .insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))])
            .get_ast(),
        vec![
            Fragment::Body(Body::from("Add file")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from(
                "This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests."
            )),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Relates-to: #128\nRelates-to: #656")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from(
                "# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here"
            )),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from(
                "# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#"
            )),
        ],
        "Inserting after the last body should append the new fragment after the last non-empty body fragment"
    );
}

#[test]
fn test_insert_after_last_body_with_no_body_inserts_at_beginning() {
    let ast: Vec<Fragment<'_>> = vec![
        Fragment::Comment(Comment::from(
            "# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here",
        )),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from(
            "# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#",
        )),
    ];
    let commit = CommitMessage::from_fragments(ast, None);

    assert_eq!(
        commit
            .insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))])
            .get_ast(),
        vec![
            Fragment::Body(Body::from("Relates-to: #656")),
            Fragment::Comment(Comment::from(
                "# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here"
            )),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from(
                "# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#"
            )),
        ],
        "When there is no body, inserting after the last body should insert at the beginning of the AST"
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn test_with_subject_preserves_input_string(input: String) -> bool {
    let commit: CommitMessage<'_> = "Some Subject".into();
    let actual: String = commit
        .with_subject(input.clone().into())
        .get_subject()
        .into();
    // Property: The subject should be exactly the input string after setting it
    actual == input
}

#[test]
fn test_with_subject_on_default_commit_sets_subject_correctly() {
    let commit = CommitMessage::default().with_subject("Subject".into());
    assert_eq!(
        commit.get_subject(),
        Subject::from("Subject"),
        "Setting subject on default commit should update the subject correctly"
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn test_with_body_contents_replaces_body_correctly(input: String) -> TestResult {
    if input.contains('\r') {
        return TestResult::discard();
    }

    let commit: CommitMessage<'_> = "Some Subject\n\nSome Body".into();
    let expected: String = format!("Some Subject\n\n{input}");
    let actual: String = commit.with_body_contents(&input).into();
    // Property: The body should be replaced with the input string while preserving the subject
    TestResult::from_bool(actual == expected)
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn test_with_body_contents_preserves_multiline_subject(input: String) -> TestResult {
    if input.contains('\r') {
        return TestResult::discard();
    }

    let commit: CommitMessage<'_> = "Some Subject\nSome More Subject\n\nBody".into();
    let expected: String = format!("Some Subject\nSome More Subject\n\n{input}");
    let actual: String = commit.with_body_contents(&input).into();
    // Property: The body should be replaced with the input string while preserving the multi-line subject
    TestResult::from_bool(actual == expected)
}

#[test]
fn test_get_comment_char_returns_none_when_no_comments() {
    let commit_character = CommitMessage::from("Example Commit Message");
    assert!(
        commit_character.get_comment_char().is_none(),
        "Comment character should be None when there are no comments in the message"
    );
}

#[test]
fn test_try_from_path_buf_reads_file_correctly() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(temp_file.as_file(), "Some Subject").expect("Failed to write file");

    let commit_character: CommitMessage<'_> = temp_file
        .path()
        .to_path_buf()
        .try_into()
        .expect("Could not read commit message");
    assert_eq!(
        commit_character.get_subject().to_string(),
        "Some Subject",
        "Reading from PathBuf should correctly parse the file contents into a CommitMessage"
    );
}

#[test]
fn test_try_from_path_reads_file_correctly() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(temp_file.as_file(), "Some Subject").expect("Failed to write file");

    let commit_character: CommitMessage<'_> = temp_file
        .path()
        .try_into()
        .expect("Could not read commit message");
    assert_eq!(
        commit_character.get_subject().to_string(),
        "Some Subject",
        "Reading from Path should correctly parse the file contents into a CommitMessage"
    );
}
