use std::{
    borrow::Cow,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::{Fragment, body::Body};

/// A [`Trailer`] you might see a in a [`CommitMessage`], for example
/// 'Co-authored-by: Billie Thompson <billie@example.com>'
#[derive(Debug, Clone, Eq, Ord, PartialOrd)]
pub struct Trailer<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    /// The separator between key and value — typically `": "` but `"#"` is
    /// also supported by `git-interpret-trailers` (e.g. `Fix #42`).
    separator: Cow<'a, str>,
}

impl<'a> Trailer<'a> {
    /// Create a new [`Trailer`]
    ///
    /// This creates a new element that represents the sort of [`Trailers`] you
    /// get at the end of commits
    ///
    /// For example there's `Co-authored-by`, `Relates-to`, and `Signed-off-by`
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by".into(), "#124".into()),
    ///     Trailer::try_from(Body::from("Co-authored-by: #124"))
    ///         .expect("There should have been a trailer in that body component")
    /// )
    /// ```
    #[must_use]
    pub const fn new(key: Cow<'a, str>, value: Cow<'a, str>) -> Self {
        Self {
            key,
            value,
            separator: Cow::Borrowed(": "),
        }
    }

    /// Create a new [`Trailer`] with a custom separator
    ///
    /// This is useful for trailers that use `#` as a separator instead of
    /// `: `, such as `Fix #42`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new_with_separator("Fix".into(), "42".into(), "#".into()),
    ///     Trailer::try_from(Body::from("Fix #42"))
    ///         .expect("There should have been a trailer in that body component")
    /// )
    /// ```
    #[must_use]
    pub const fn new_with_separator(
        key: Cow<'a, str>,
        value: Cow<'a, str>,
        separator: Cow<'a, str>,
    ) -> Self {
        Self {
            key,
            value,
            separator,
        }
    }

    /// Get the key of the [`Trailer`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by".into(), "#124".into()).get_key(),
    ///     "Co-authored-by"
    /// )
    /// ```
    #[must_use]
    pub fn get_key(&self) -> String {
        format!("{}", self.key)
    }

    /// Get the value of the [`Trailer`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use mit_commit::{Body, Trailer};
    /// assert_eq!(
    ///     Trailer::new("Co-authored-by".into(), "#124".into()).get_value(),
    ///     "#124"
    /// )
    /// ```
    #[must_use]
    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

impl PartialEq for Trailer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value.trim_end() == other.value.trim_end()
    }
}

impl Hash for Trailer<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.trim_end().hash(state);
    }
}

impl From<Trailer<'_>> for String {
    fn from(trailer: Trailer<'_>) -> Self {
        format!("{}{}{}", trailer.key, trailer.separator, trailer.value)
    }
}

impl<'a> From<Trailer<'a>> for Fragment<'a> {
    fn from(trailer: Trailer<'_>) -> Self {
        let trailer: String = trailer.into();
        Body::from(trailer).into()
    }
}

impl<'a> TryFrom<Body<'a>> for Trailer<'a> {
    type Error = Error;

    fn try_from(body: Body<'a>) -> Result<Self, Self::Error> {
        let content: String = body.into();

        // The canonical trailer form is "Key: value" (colon-space), which
        // covers Signed-off-by, Co-authored-by, Acked-by, See-also, etc.
        if let Some(trailer) = parse_colon_trailer(&content) {
            return Ok(trailer);
        }

        // git-interpret-trailers also supports "#" as a separator, producing
        // trailers like "Fix #42".  This is less common and more prone to
        // false positives (any line containing "#" could match), so we
        // restrict it to short, word-like keys.
        //
        // See: https://git-scm.com/docs/git-interpret-trailers
        if let Some(trailer) = parse_hash_trailer(&content) {
            return Ok(trailer);
        }

        Err(Error::new_not_a_trailer(&content))
    }
}

/// Parse a colon-separated trailer: "Key: value".
fn parse_colon_trailer<'a>(content: &str) -> Option<Trailer<'a>> {
    let (key, value) = content.split_once(": ")?;

    if key.is_empty() {
        return None;
    }

    Some(Trailer::new(key.to_owned().into(), value.to_owned().into()))
}

/// Parse a hash-separated trailer: "Fix #42".
///
/// The key must be a short token (letters, digits, hyphens) with no spaces,
/// to avoid matching arbitrary body text that happens to contain "#".
fn parse_hash_trailer<'a>(content: &str) -> Option<Trailer<'a>> {
    let mut parts = content.splitn(2, '#');
    let key = parts.next()?.trim();
    let value = parts.next()?;

    if key.is_empty() {
        return None;
    }

    // Only accept word-like keys (e.g. "Fix", "Fixes", "Closes") to prevent
    // false positives on body prose containing "#".
    if !key.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return None;
    }

    Some(Trailer::new_with_separator(
        key.to_owned().into(),
        value.to_owned().into(),
        " #".into(),
    ))
}

/// Errors in parsing potential trailers
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// When the given fragment is not a trailer
    #[error("not a trailer")]
    #[diagnostic(url(docsrs), code(mit_commit::trailer::error::not_atrailer))]
    NotATrailer(
        #[source_code] String,
        #[label("no colon in body line")] (usize, usize),
    ),
}

impl Error {
    fn new_not_a_trailer(text: &str) -> Self {
        Self::NotATrailer(text.to_string(), (0, text.len()))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    };

    use super::Trailer;
    use crate::{Fragment, body::Body};

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
    fn it_does_not_match_on_differing_vales() {
        let a = Trailer::new("Relates-to".into(), "#129".into());
        let b = Trailer::new("Relates-to".into(), "#128".into());

        assert_ne!(a, b);
    }

    #[test]
    fn it_does_not_match_on_differing_names() {
        let a = Trailer::new("Another".into(), "#128".into());
        let b = Trailer::new("Relates-to".into(), "#128".into());

        assert_ne!(a, b);
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
    fn it_differing_relates_headers_do_not_match_hashes() {
        let mut hasher_a = DefaultHasher::new();
        Trailer::new("Relates".into(), "#128".into()).hash(&mut hasher_a);

        let mut hasher_b = DefaultHasher::new();
        Trailer::new("Relates-to".into(), "#128".into()).hash(&mut hasher_b);

        assert_ne!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn it_differing_relates_values_do_not_match_hashes() {
        let mut hasher_a = DefaultHasher::new();
        Trailer::new("Relates-to".into(), "#129".into()).hash(&mut hasher_a);

        let mut hasher_b = DefaultHasher::new();
        Trailer::new("Relates-to".into(), "#128".into()).hash(&mut hasher_b);

        assert_ne!(hasher_a.finish(), hasher_b.finish());
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

    #[test]
    fn it_preserves_value_containing_colon_space() {
        let trailer = Trailer::try_from(Body::from(
            "See: fix crash in http://example.com:8080 handler",
        ))
        .expect("Should parse as a trailer");

        assert_eq!(
            String::from(trailer),
            String::from("See: fix crash in http://example.com:8080 handler"),
            "Trailer value containing ': ' should be preserved in full"
        );
    }

    #[test]
    fn it_preserves_value_with_multiple_colon_spaces() {
        let trailer = Trailer::try_from(Body::from(
            "Co-authored-by: Someone <someone@example.com>: extra",
        ))
        .expect("Should parse as a trailer");

        assert_eq!(
            trailer.get_key(),
            "Co-authored-by",
            "Key should only be the part before the first ': '"
        );
        assert_eq!(
            trailer.get_value(),
            "Someone <someone@example.com>: extra",
            "Value should include everything after the first ': ', including subsequent ': '"
        );
    }

    #[test]
    fn it_parses_hash_separator_trailer() {
        let trailer = Trailer::try_from(Body::from("Fix #42")).expect("Should parse as a trailer");

        assert_eq!(trailer.get_key(), "Fix");
        assert_eq!(trailer.get_value(), "42");
    }

    #[test]
    fn it_round_trips_hash_separator_trailer() {
        let trailer = Trailer::try_from(Body::from("Fix #42")).expect("Should parse as a trailer");

        assert_eq!(
            String::from(trailer),
            String::from("Fix #42"),
            "Hash separator trailer should round-trip exactly"
        );
    }

    #[test]
    fn it_parses_fixes_hash_trailer() {
        let trailer =
            Trailer::try_from(Body::from("Fixes #123")).expect("Should parse as a trailer");

        assert_eq!(trailer.get_key(), "Fixes");
        assert_eq!(trailer.get_value(), "123");
    }

    #[test]
    fn it_parses_closes_hash_trailer() {
        let trailer =
            Trailer::try_from(Body::from("Closes #7")).expect("Should parse as a trailer");

        assert_eq!(trailer.get_key(), "Closes");
        assert_eq!(trailer.get_value(), "7");
    }

    #[test]
    fn it_does_not_parse_prose_with_hash_as_trailer() {
        // A sentence containing "#" should not be mistaken for a trailer
        // because the "key" part contains spaces.
        let result = Trailer::try_from(Body::from("This is a sentence with a #hashtag in it"));

        assert!(
            result.is_err(),
            "Prose containing '#' should not be parsed as a hash trailer"
        );
    }

    #[test]
    fn it_round_trips_hash_trailer_from_new_with_separator() {
        let trailer = Trailer::new_with_separator("Fix".into(), "42".into(), " #".into());

        assert_eq!(String::from(trailer), String::from("Fix #42"));
    }

    #[test]
    fn hash_and_colon_trailers_with_same_key_value_are_equal() {
        // Two trailers with the same key and value are equal regardless
        // of separator — the separator is a formatting concern, not identity.
        let hash_trailer = Trailer::new_with_separator("Fix".into(), "42".into(), " #".into());
        let colon_trailer = Trailer::new("Fix".into(), "42".into());

        assert_eq!(hash_trailer, colon_trailer);
    }
}
