use crate::{Body, Comment, Fragment};

#[test]
fn can_convert_body_into_a_fragment() {
    let body: Body<'_> = "A Body".into();
    let fragment: Fragment<'_> = body.clone().into();

    assert_eq!(fragment, Fragment::Body(body));
}

#[test]
fn can_convert_comment_into_a_fragment() {
    let comment: Comment<'_> = "A Comment".into();
    let fragment: Fragment<'_> = comment.clone().into();

    assert_eq!(fragment, Fragment::Comment(comment));
}
