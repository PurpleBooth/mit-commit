use indoc::indoc;
use mit_commit::{
    Bodies,
    Body,
    Comment,
    Comments,
    CommitMessage,
    Fragment,
    Scissors,
    Subject,
    Trailer,
    Trailers,
};

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
