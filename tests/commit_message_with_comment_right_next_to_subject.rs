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
