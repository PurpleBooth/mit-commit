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
