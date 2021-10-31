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
        Trailer::new(
            "Co-authored-by".into(),
            "Billie Thomposon <billie@example.com>".into(),
        ),
        Trailer::new(
            " Co-authored-by".into(),
            "Somebody Else <somebody@example.com>".into(),
        ),
    ];

    assert_eq!(message.get_trailers(), Trailers::from(trailers));
}
