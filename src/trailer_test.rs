use std::{
    collections::hash_map::DefaultHasher,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

use crate::{body::Body, Fragment};

use super::Trailer;

#[test]
fn it_can_tell_me_its_key() {
    let trailer = Trailer::new("Relates-to".into(), "#128".into());

    assert_eq!(trailer.get_key(), String::from("Relates-to"));
}

#[test]
fn it_can_tell_me_its_value() {
    let trailer = Trailer::new("Relates-to".into(), "#128".into());

    assert_eq!(trailer.get_value(), String::from("#128"));
}

#[test]
fn it_does_not_take_trailing_whitespace_into_account_in_equality_checks() {
    let a = Trailer::new("Relates-to".into(), "#128\n".into());
    let b = Trailer::new("Relates-to".into(), "#128".into());

    assert_eq!(a, b);
}

#[test]
fn it_does_not_take_trailing_whitespace_into_account_in_hashing() {
    let mut hasher_a = DefaultHasher::new();
    Trailer::new("Relates-to".into(), "#128\n".into()).hash(&mut hasher_a);

    let mut hasher_b = DefaultHasher::new();
    Trailer::new("Relates-to".into(), "#128".into()).hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn it_can_give_me_itself_as_a_string() {
    let trailer = Trailer::new("Relates-to".into(), "#128".into());

    assert_eq!(String::from(trailer), String::from("Relates-to: #128"));
}

#[test]
fn can_generate_itself_from_body() {
    let trailer = Trailer::try_from(Body::from("Relates-to: #128"));

    assert_eq!(
        String::from(trailer.expect("Could not parse from string")),
        String::from("Relates-to: #128")
    );
}

#[test]
fn it_preserves_preceding_whitespace() {
    let trailer = Trailer::try_from(Body::from("Relates-to:      #128\n"));

    assert_eq!(
        String::from(trailer.expect("Could not parse from string")),
        String::from("Relates-to:      #128\n")
    );
}

#[test]
fn can_generate_from_body() {
    let trailer = Trailer::new("Relates-to".into(), "#128".into());
    let body: Fragment<'_> = Fragment::from(trailer);

    assert_eq!(body, Fragment::Body(Body::from("Relates-to: #128")));
}
