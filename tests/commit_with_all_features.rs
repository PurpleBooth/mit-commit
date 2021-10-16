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
