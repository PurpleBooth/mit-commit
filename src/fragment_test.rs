use std::borrow::Borrow;
use crate::{Body, Comment, Fragment};

#[test]
fn can_convert_body_into_a_fragment() {
    let body: Body = "A Body".into();
    let fragment: Fragment = body.clone().into();

    assert_eq!(fragment, Fragment::Body(body));
}

#[test]
fn can_convert_reference_body_into_a_fragment() {
    let body: Body = "A Body".into();
    let fragment: Fragment = body.borrow().into();

    assert_eq!(fragment, Fragment::Body(body));
}


#[test]
fn can_convert_comment_into_a_fragment() {
    let comment: Comment = "A Comment".into();
    let fragment: Fragment = comment.clone().into();

    assert_eq!(fragment, Fragment::Comment(comment));
}

#[test]
fn can_convert_reference_comment_into_a_fragment() {
    let comment: Comment = "A Comment".into();
    let fragment: Fragment = comment.borrow().into();

    assert_eq!(fragment, Fragment::Comment(comment));
}
