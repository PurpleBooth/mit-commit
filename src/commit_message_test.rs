use indoc::indoc;
use quickcheck::TestResult;
use regex::Regex;

use super::CommitMessage;
use crate::{
    bodies::Bodies,
    body::Body,
    comment::Comment,
    comments::Comments,
    scissors::Scissors,
    subject::Subject,
    trailer::Trailer,
    trailers::Trailers,
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
        String::from(commit.add_trailer(Trailer::new("Co-authored-by", "Test Trailer <test@example.com>"))),
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
            "Co-authored-by",
            "Test Trailer <test@example.com>",
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
            "Co-authored-by",
            "Test Trailer <test@example.com>",
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
            "Co-authored-by",
            "Someone Else <someone@example.com>",
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
    let ast: Vec<Fragment> = vec![
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
    let ast: Vec<Fragment> = vec![
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

const COMMIT_WITH_ALL_FEATURES: &str = indoc!(
    "
    Add file

    Looks-like-a-trailer: But isn't

    This adds file primarily for demonstration purposes. It might not be
    useful as an actual commit, but it's very useful as a example to use in
    tests.

    Relates-to: #128

    # Short (50 chars or less) summary of changes
    #
    # More detailed explanatory text, if necessary.  Wrap it to
    # about 72 characters or so.  In some contexts, the first
    # line is treated as the subject of an email and the rest of
    # the text as the body.  The blank line separating the
    # summary from the body is critical (unless you omit the body
    # entirely); tools like rebase can get confused if you run
    # the two together.
    #
    # Further paragraphs come after blank lines.
    #
    #   - Bullet points are okay, too
    #
    #   - Typically a hyphen or asterisk is used for the bullet,
    #     preceded by a single space, with blank lines in
    #     between, but conventions vary here

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
    # ------------------------ >8 ------------------------
    # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    # Alles unterhalb von ihr wird ignoriert.
    diff --git a/file b/file
    new file mode 100644
    index 0000000..e69de29
    "
);

#[test]
fn can_reliably_parse_from_commit_with_all_features() {
    let first_commit_message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, COMMIT_WITH_ALL_FEATURES);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_comment_character_with_all_features() {
    let commit_character = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);
    assert_eq!(commit_character.get_comment_char().unwrap(), '#');
}

#[test]
fn can_get_ast_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);
    let ast: Vec<Fragment> = vec![
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

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

    assert_eq!(message.get_subject(), Subject::from("Add file"));
}

#[test]
fn can_get_body_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from("Looks-like-a-trailer: But isn't"),
            Body::default(),
            Body::from(indoc!(
                "
                This adds file primarily for demonstration purposes. It might not be
                useful as an actual commit, but it's very useful as a example to use in
                tests."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

    assert_eq!(
        message.get_scissors(),
        Some(Scissors::from(indoc!(
            "
            # ------------------------ >8 ------------------------
            # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            new file mode 100644
            index 0000000..e69de29
            "
        )))
    );
}

#[test]
fn can_get_comments_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![
            Comment::from(indoc!(
                    "
                    # Short (50 chars or less) summary of changes
                    #
                    # More detailed explanatory text, if necessary.  Wrap it to
                    # about 72 characters or so.  In some contexts, the first
                    # line is treated as the subject of an email and the rest of
                    # the text as the body.  The blank line separating the
                    # summary from the body is critical (unless you omit the body
                    # entirely); tools like rebase can get confused if you run
                    # the two together.
                    #
                    # Further paragraphs come after blank lines.
                    #
                    #   - Bullet points are okay, too
                    #
                    #   - Typically a hyphen or asterisk is used for the bullet,
                    #     preceded by a single space, with blank lines in
                    #     between, but conventions vary here"
                )),
            Comment::from(indoc!(
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
                    #"
                )),
        ])
    );
}

#[test]
fn can_get_trailers_from_commit_with_all_features() {
    let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

    assert_eq!(
        message.get_trailers(),
        Trailers::from(vec![Trailer::new("Relates-to", "#128")])
    );
}

const LONG_SUBJECT_ONLY_COMMIT: &str = indoc!(
    "
    Initial Commit
    # Short (50 chars or less) summary of changes
    #
    # More detailed explanatory text, if necessary.  Wrap it to
    # about 72 characters or so.  In some contexts, the first
    # line is treated as the subject of an email and the rest of
    # the text as the body.  The blank line separating the
    # summary from the body is critical (unless you omit the body
    # entirely); tools like rebase can get confused if you run
    # the two together.
    #
    # Further paragraphs come after blank lines.
    #
    #   - Bullet points are okay, too
    #
    #   - Typically a hyphen or asterisk is used for the bullet,
    #     preceded by a single space, with blank lines in
    #     between, but conventions vary here

    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
    # bricht den Commit ab.
    #
    # Auf Branch master
    #
    # Initialer Commit
    #
    # Zum Commit vorgemerkte \u{00E4}nderungen:
    #	neue Datei:     src/bodies.rs
    #	neue Datei:     src/body.rs
    #	neue Datei:     src/comment.rs
    #	neue Datei:     src/comments.rs
    #	neue Datei:     src/commit_message.rs
    #	neue Datei:     src/scissors.rs
    #	neue Datei:     src/subject.rs
    #	neue Datei:     src/trailer.rs
    #	neue Datei:     src/trailers.rs
    #
    # \u{00E4}nderungen, die nicht zum Commit vorgemerkt sind:
    #	ge\u{00E4}ndert:       src/bodies.rs
    #	ge\u{00E4}ndert:       src/body.rs
    #	ge\u{00E4}ndert:       src/comment.rs
    #	ge\u{00E4}ndert:       src/comments.rs
    #	ge\u{00E4}ndert:       src/commit_message.rs
    #	ge\u{00E4}ndert:       src/scissors.rs
    #	ge\u{00E4}ndert:       src/subject.rs
    #	ge\u{00E4}ndert:       src/trailer.rs
    #	ge\u{00E4}ndert:       src/trailers.rs
    #
    # Unversionierte Dateien:
    #	.gitignore
    #	Cargo.toml
    #	src/lib.rs
    #
    # ------------------------ >8 ------------------------
    # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    # Alles unterhalb von ihr wird ignoriert.
    diff --git a/src/bodies.rs b/src/bodies.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/body.rs b/src/body.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/comment.rs b/src/comment.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/comments.rs b/src/comments.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/commit_message.rs b/src/commit_message.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/scissors.rs b/src/scissors.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/subject.rs b/src/subject.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/trailer.rs b/src/trailer.rs
    new file mode 100644
    index 0000000..e69de29
    diff --git a/src/trailers.rs b/src/trailers.rs
    new file mode 100644
    index 0000000..e69de29"
);

#[test]
fn can_reliably_parse_from_subject_only_commit() {
    let first_commit_message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, LONG_SUBJECT_ONLY_COMMIT);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Initial Commit")),
        Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     src/bodies.rs\n#\tneue Datei:     src/body.rs\n#\tneue Datei:     src/comment.rs\n#\tneue Datei:     src/comments.rs\n#\tneue Datei:     src/commit_message.rs\n#\tneue Datei:     src/scissors.rs\n#\tneue Datei:     src/subject.rs\n#\tneue Datei:     src/trailer.rs\n#\tneue Datei:     src/trailers.rs\n#\n# \u{e4}nderungen, die nicht zum Commit vorgemerkt sind:\n#\tge\u{e4}ndert:       src/bodies.rs\n#\tge\u{e4}ndert:       src/body.rs\n#\tge\u{e4}ndert:       src/comment.rs\n#\tge\u{e4}ndert:       src/comments.rs\n#\tge\u{e4}ndert:       src/commit_message.rs\n#\tge\u{e4}ndert:       src/scissors.rs\n#\tge\u{e4}ndert:       src/subject.rs\n#\tge\u{e4}ndert:       src/trailer.rs\n#\tge\u{e4}ndert:       src/trailers.rs\n#\n# Unversionierte Dateien:\n#\t.gitignore\n#\tCargo.toml\n#\tsrc/lib.rs\n#")),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_comment_character_only_commit() {
    let commit_character = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
    assert_eq!(commit_character.get_comment_char().unwrap(), '#');
}

#[test]
fn can_get_subject_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

    assert_eq!(message.get_subject(), Subject::from("Initial Commit"));
}

#[test]
fn can_get_body_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
    let bodies: Vec<Body> = vec![];

    assert_eq!(message.get_body(), Bodies::from(bodies));
}

#[test]
fn can_get_scissors_section_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

    assert_eq!(
        message.get_scissors(),
        Some(Scissors::from(indoc!(
            "
            # ------------------------ >8 ------------------------
            # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/src/bodies.rs b/src/bodies.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/body.rs b/src/body.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/comment.rs b/src/comment.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/comments.rs b/src/comments.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/commit_message.rs b/src/commit_message.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/scissors.rs b/src/scissors.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/subject.rs b/src/subject.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/trailer.rs b/src/trailer.rs
            new file mode 100644
            index 0000000..e69de29
            diff --git a/src/trailers.rs b/src/trailers.rs
            new file mode 100644
            index 0000000..e69de29"
        )))
    );
}

#[test]
fn can_get_comments_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![
            Comment::from(indoc!(
                    "
                    # Short (50 chars or less) summary of changes
                    #
                    # More detailed explanatory text, if necessary.  Wrap it to
                    # about 72 characters or so.  In some contexts, the first
                    # line is treated as the subject of an email and the rest of
                    # the text as the body.  The blank line separating the
                    # summary from the body is critical (unless you omit the body
                    # entirely); tools like rebase can get confused if you run
                    # the two together.
                    #
                    # Further paragraphs come after blank lines.
                    #
                    #   - Bullet points are okay, too
                    #
                    #   - Typically a hyphen or asterisk is used for the bullet,
                    #     preceded by a single space, with blank lines in
                    #     between, but conventions vary here"
                )),
            Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     src/bodies.rs
                    #	neue Datei:     src/body.rs
                    #	neue Datei:     src/comment.rs
                    #	neue Datei:     src/comments.rs
                    #	neue Datei:     src/commit_message.rs
                    #	neue Datei:     src/scissors.rs
                    #	neue Datei:     src/subject.rs
                    #	neue Datei:     src/trailer.rs
                    #	neue Datei:     src/trailers.rs
                    #
                    # \u{00E4}nderungen, die nicht zum Commit vorgemerkt sind:
                    #	ge\u{00E4}ndert:       src/bodies.rs
                    #	ge\u{00E4}ndert:       src/body.rs
                    #	ge\u{00E4}ndert:       src/comment.rs
                    #	ge\u{00E4}ndert:       src/comments.rs
                    #	ge\u{00E4}ndert:       src/commit_message.rs
                    #	ge\u{00E4}ndert:       src/scissors.rs
                    #	ge\u{00E4}ndert:       src/subject.rs
                    #	ge\u{00E4}ndert:       src/trailer.rs
                    #	ge\u{00E4}ndert:       src/trailers.rs
                    #
                    # Unversionierte Dateien:
                    #	.gitignore
                    #	Cargo.toml
                    #	src/lib.rs
                    #"
                )),
        ])
    );
}

#[test]
fn can_get_trailers_from_subject_only_commit() {
    let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
    let trailers: Vec<Trailer> = Vec::default();

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const NOT_VERBOSE_COMMIT: &str = indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
    # bricht den Commit ab.
    #
    # Datum:            Sat Jun 27 21:40:14 2020 +0200
    #
    # Auf Branch master
    #
    # Initialer Commit
    #
    # Zum Commit vorgemerkte \u{00E4}nderungen:
    #	neue Datei:     .bashrc
    #"
);

#[test]
fn can_reliably_parse_from_not_verbose_commit() {
    let first_commit_message = CommitMessage::from(NOT_VERBOSE_COMMIT);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, NOT_VERBOSE_COMMIT);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_comment_character_not_verbose_commit() {
    let commit_character = CommitMessage::from(NOT_VERBOSE_COMMIT);
    assert_eq!(commit_character.get_comment_char().unwrap(), '#');
}

#[test]
fn can_get_ast_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                "
                This should make it easier to deploy things for the developers.
                Benchmarked with Hyperfine, no noticable performance decrease."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![
            Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
        ])
    );
}

#[test]
fn can_get_trailers_from_not_verbose_commit() {
    let message = CommitMessage::from(NOT_VERBOSE_COMMIT);
    let trailers: Vec<Trailer> = Vec::default();

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const NON_STANDARD_COMMENT_CHARACTER: &str = indoc!(
    "
    Allow the server to respond to https

    This allows the server to respond to HTTPS requests, by correcting the port binding.
    We should see a nice speed increase from this

    fixes:
    #6436
    #6437
    #6438

    ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
    ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ; bricht den Commit ab."
);

#[test]
fn can_reliably_parse_from_non_standard_comment_char_commit() {
    let first_commit_message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, NON_STANDARD_COMMENT_CHARACTER);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_comment_character_non_standard_comment_char_commit() {
    let commit_character = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
    assert_eq!(commit_character.get_comment_char().unwrap(), ';');
}

#[test]
fn can_get_ast_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Allow the server to respond to https")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This allows the server to respond to HTTPS requests, by correcting the port binding.\nWe should see a nice speed increase from this")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("fixes:\n#6436\n#6437\n#6438")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("; Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{c4}nderungen ein. Zeilen,\n; die mit \';\' beginnen, werden ignoriert, und eine leere Beschreibung\n; bricht den Commit ab.")),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

    assert_eq!(
        message.get_subject(),
        Subject::from("Allow the server to respond to https")
    );
}

#[test]
fn can_get_body_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                    "
                    This allows the server to respond to HTTPS requests, by correcting the port binding.
                    We should see a nice speed increase from this"
                    )),
            Body::default(),
            Body::from(indoc!(
                    "
                    fixes:
                    #6436
                    #6437
                    #6438"
                )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![Comment::from(indoc!(
                "
                ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
                ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
                ; bricht den Commit ab."
            ))])
    );
}

#[test]
fn can_get_trailers_from_non_standard_comment_char_commit() {
    let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
    let trailers: Vec<Trailer> = Vec::default();

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST: &str = indoc!(
    "
    Allow the server to respond to https

    This allows the server to respond to HTTPS requests, by correcting the port binding.
    We should see a nice speed increase from this

    fixes:
    #6436
    #6437
    #6438

    ? Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
    ? die mit '?' beginnen, werden ignoriert, und eine leere Beschreibung
    ? bricht den Commit ab."
);

#[test]
fn can_reliably_parse_from_comment_char_that_is_not_in_legal_list() {
    let first_commit_message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Allow the server to respond to https")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This allows the server to respond to HTTPS requests, by correcting the port binding.\nWe should see a nice speed increase from this")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("fixes:\n#6436\n#6437\n#6438")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("? Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{c4}nderungen ein. Zeilen,\n? die mit \'?\' beginnen, werden ignoriert, und eine leere Beschreibung\n? bricht den Commit ab.")),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);

    assert_eq!(
        message.get_subject(),
        Subject::from("Allow the server to respond to https")
    );
}

#[test]
fn can_get_body_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                    "
                    This allows the server to respond to HTTPS requests, by correcting the port binding.
                    We should see a nice speed increase from this"
                    )),
            Body::default(),
            Body::from(indoc!(
                    "
                    fixes:
                    #6436
                    #6437
                    #6438"
                )),
            Body::default(),
            Body::from("? Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{c4}nderungen ein. Zeilen,\n? die mit \'?\' beginnen, werden ignoriert, und eine leere Beschreibung\n? bricht den Commit ab."),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);

    assert_eq!(
        message.get_comments(),
        Comments::from(Vec::<Comment>::new())
    );
}

#[test]
fn can_get_trailers_from_comment_char_that_is_not_in_legal_list() {
    let message = CommitMessage::from(COMMENT_CHAR_IS_NOT_IN_LEGAL_LIST);
    let trailers: Vec<Trailer> = Vec::default();

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const MULTIPLE_TRAILERS: &str = indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    Co-authored-by: Billie Thomposon <billie@example.com>
    Co-authored-by: Somebody Else <somebody@example.com>

    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
    # bricht den Commit ab.
    #
    # Datum:            Sat Jun 27 21:40:14 2020 +0200
    #
    # Auf Branch master
    #
    # Initialer Commit
    #
    # Zum Commit vorgemerkte \u{00E4}nderungen:
    #	neue Datei:     .bashrc
    #"
);

#[test]
fn can_reliably_parse_from_multiple_trailers() {
    let first_commit_message = CommitMessage::from(MULTIPLE_TRAILERS);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, MULTIPLE_TRAILERS);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                "
                This should make it easier to deploy things for the developers.
                Benchmarked with Hyperfine, no noticable performance decrease."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![
            Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
        ])
    );
}

#[test]
fn can_get_trailers_from_multiple_trailers() {
    let message = CommitMessage::from(MULTIPLE_TRAILERS);
    let trailers: Vec<Trailer> = vec![
        Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
        Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
    ];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const TRAILING_EMPTY_NEWLINES: &str = indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    Co-authored-by: Billie Thomposon <billie@example.com>
    Co-authored-by: Somebody Else <somebody@example.com>

    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
    # bricht den Commit ab.
    #
    # Datum:            Sat Jun 27 21:40:14 2020 +0200
    #
    # Auf Branch master
    #
    # Initialer Commit
    #
    # Zum Commit vorgemerkte \u{00E4}nderungen:
    #	neue Datei:     .bashrc
    #


    "
);

#[test]
fn can_reliably_parse_from_trailing_empty_newlines() {
    let first_commit_message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, TRAILING_EMPTY_NEWLINES);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
        Fragment::Body(Body::default()),
        Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::default()),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                "
                This should make it easier to deploy things for the developers.
                Benchmarked with Hyperfine, no noticable performance decrease."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

    assert_eq!(
        message.get_comments(),
        Comments::from(vec![
            Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
        ])
    );
}

#[test]
fn can_get_trailers_from_trailing_empty_newlines() {
    let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
    let trailers: Vec<Trailer> = vec![
        Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
        Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
    ];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const COMMIT_MESSAGE_WITH_NO_COMMENTS: &str = indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    Co-authored-by: Billie Thomposon <billie@example.com>
    Co-authored-by: Somebody Else <somebody@example.com>
    "
);

#[test]
fn can_reliably_parse_from_a_commit_message_without_commits() {
    let first_commit_message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(string_version_of_commit, COMMIT_MESSAGE_WITH_NO_COMMENTS);
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
        Fragment::Body(Body::default()),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                "
                This should make it easier to deploy things for the developers.
                Benchmarked with Hyperfine, no noticable performance decrease."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
    let comments: Vec<Comment> = vec![];

    assert_eq!(message.get_comments(), Comments::from(comments));
}

#[test]
fn can_get_trailers_from_a_commit_message_without_commits() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
    let trailers: Vec<Trailer> = vec![
        Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
        Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
    ];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE: &str = indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    Co-authored-by: Billie Thomposon <billie@example.com>
     Co-authored-by: Somebody Else <somebody@example.com>
    "
);

#[test]
fn can_reliably_parse_from_a_commit_message_with_trailing_whitespace() {
    let first_commit_message =
        CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(
        string_version_of_commit,
        COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE
    );
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
        Fragment::Body(Body::default()),
        Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\n Co-authored-by: Somebody Else <somebody@example.com>")),
        Fragment::Body(Body::default()),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

    assert_eq!(
        message.get_body(),
        Bodies::from(vec![
            Body::default(),
            Body::from(indoc!(
                "
                This should make it easier to deploy things for the developers.
                Benchmarked with Hyperfine, no noticable performance decrease."
            )),
        ])
    );
}

#[test]
fn can_get_scissors_section_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
    let comments: Vec<Comment> = vec![];

    assert_eq!(message.get_comments(), Comments::from(comments));
}

#[test]
fn can_get_trailers_from_a_commit_message_with_trailing_whitespace() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
    let trailers: Vec<Trailer> = vec![
        Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
        Trailer::new(" Co-authored-by", "Somebody Else <somebody@example.com>"),
    ];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

const COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT: &str = indoc!(
    "
    Update bashrc to include kubernetes completions
    # Comment here
    "
);

#[test]
fn can_reliably_parse_from_a_commit_message_with_comment_right_next_to_subject() {
    let first_commit_message =
        CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);
    let string_version_of_commit = String::from(first_commit_message.clone());
    let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

    assert_eq!(
        string_version_of_commit,
        COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT
    );
    assert_eq!(first_commit_message, second_commit_message);
}

#[test]
fn can_get_ast_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);
    let ast: Vec<Fragment> = vec![
        Fragment::Body(Body::from(
            "Update bashrc to include kubernetes completions",
        )),
        Fragment::Comment(Comment::from("# Comment here")),
        Fragment::Body(Body::default()),
    ];

    assert_eq!(message.get_ast(), ast);
}

#[test]
fn can_get_subject_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);

    assert_eq!(
        message.get_subject(),
        Subject::from("Update bashrc to include kubernetes completions")
    );
}

#[test]
fn can_get_body_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);
    assert_eq!(message.get_body(), Bodies::default());
}

#[test]
fn can_get_scissors_section_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);

    assert_eq!(message.get_scissors(), None);
}

#[test]
fn can_get_comments_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);
    let comments: Vec<Comment> = vec![Comment::from("# Comment here")];

    assert_eq!(message.get_comments(), Comments::from(comments));
}

#[test]
fn can_get_trailers_from_a_commit_message_with_comment_right_next_to_subject() {
    let message = CommitMessage::from(COMMIT_MESSAGE_WITH_COMMENT_RIGHT_NEXT_TO_SUBJECT);
    let trailers: Vec<Trailer> = vec![];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn with_subject(input: String) -> bool {
    let commit: CommitMessage = "Some Subject".into();
    let actual: String = commit.with_subject(&input).get_subject().into();
    actual == input
}

#[test]
fn with_subject_on_default_commit() {
    let commit = CommitMessage::default().with_subject("Subject");
    assert_eq!(commit.get_subject(), Subject::from("Subject"));
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn with_body(input: String) -> TestResult {
    if input.contains('\r') {
        return TestResult::discard();
    }

    let commit: CommitMessage = "Some Subject\n\nSome Body".into();
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

    let commit: CommitMessage = "Some Subject\nSome More Subject\n\nBody".into();
    let expected: String = format!("Some Subject\nSome More Subject\n\n{}", input);
    let actual: String = commit.with_body_contents(&input).into();
    TestResult::from_bool(actual == expected)
}

#[allow(unused_must_use)]
#[quickcheck]
fn never_panic(input: String) -> bool {
    CommitMessage::from(input);
    true
}

#[test]
fn can_get_comment_character_when_there_is_no_comments() {
    let commit_character = CommitMessage::from("Example Commit Message");
    assert!(commit_character.get_comment_char().is_none());
}
