use indoc::indoc;
use mit_commit::{
    Bodies,
    Body,
    Comment,
    Comments,
    CommitMessage,
    Fragment,
    Subject,
    Trailer,
    Trailers,
};

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
