use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
    str::Chars,
};

use crate::{body::Body, fragment::Fragment};

/// The [`Subject`] from the [`crate::CommitMessage`]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Subject<'a> {
    text: Cow<'a, str>,
}

impl Subject<'_> {
    /// Count characters in [`Self`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Subject;
    ///
    /// assert_eq!(Subject::from("hello, world!").len(), 13);
    /// assert_eq!(Subject::from("goodbye").len(), 7)
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Is the [`Self`] empty
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Subject;
    ///
    /// assert_eq!(Subject::from("hello, world!").is_empty(), false);
    /// assert_eq!(Subject::from("").is_empty(), true)
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Convert the [`Self`] into chars
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Subject;
    ///
    /// let subject = Subject::from("y\u{306}");
    ///
    /// let mut chars = subject.chars();
    ///
    /// assert_eq!(Some('y'), chars.next());
    /// assert_eq!(Some('\u{0306}'), chars.next());
    ///
    /// assert_eq!(None, chars.next());
    /// ```
    pub fn chars(&self) -> Chars<'_> {
        self.text.chars()
    }
}

impl<'a> From<&'a str> for Subject<'a> {
    fn from(subject: &'a str) -> Self {
        Self {
            text: subject.into(),
        }
    }
}

impl From<String> for Subject<'_> {
    /// Convert from an owned string
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Subject;
    ///
    /// let subject = Subject::from("y\u{306}".to_string());
    ///
    /// let mut chars = subject.chars();
    ///
    /// assert_eq!(Some('y'), chars.next());
    /// assert_eq!(Some('\u{0306}'), chars.next());
    /// assert_eq!(None, chars.next());
    /// ```
    fn from(subject: String) -> Self {
        Self {
            text: subject.into(),
        }
    }
}

impl<'a> From<Cow<'a, str>> for Subject<'a> {
    fn from(subject: Cow<'a, str>) -> Self {
        Self { text: subject }
    }
}

impl From<Subject<'_>> for String {
    fn from(subject: Subject<'_>) -> Self {
        subject.text.into_owned()
    }
}

impl Display for Subject<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl<'a> From<Body<'a>> for Subject<'a> {
    fn from(body: Body<'_>) -> Self {
        Self::from(String::from(body))
    }
}

impl<'a> From<Vec<Fragment<'a>>> for Subject<'a> {
    fn from(ast: Vec<Fragment<'a>>) -> Self {
        ast.iter()
            .find_map(|values| {
                if let Fragment::Body(body) = values {
                    Some(Self::from(body.clone()))
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::Subject;
    use crate::{Comment, body::Body, fragment::Fragment};

    #[test]
    fn test_subject_length_returns_correct_character_count() {
        assert_eq!(
            Subject::from("hello, world!").len(),
            13,
            "Subject length should count all characters correctly"
        );
        assert_eq!(
            Subject::from("goodbye").len(),
            7,
            "Subject length should count all characters correctly"
        );
    }

    #[test]
    fn test_chars_iterator_returns_correct_unicode_characters() {
        let subject = Subject::from("y\u{306}");

        let mut chars = subject.chars();

        assert_eq!(Some('y'), chars.next(), "First character should be 'y'");
        assert_eq!(
            Some('\u{0306}'),
            chars.next(),
            "Second character should be the combining breve (U+0306)"
        );
        assert_eq!(
            None,
            chars.next(),
            "Iterator should be exhausted after two characters"
        );
    }

    #[test]
    fn test_is_empty_returns_correct_boolean_value() {
        assert!(
            !Subject::from("hello, world!").is_empty(),
            "Non-empty subject should return false for is_empty()"
        );
        assert!(
            Subject::from("").is_empty(),
            "Empty subject should return true for is_empty()"
        );
    }

    #[test]
    fn test_display_trait_formats_subject_correctly() {
        let _subject = String::from(Subject::from("hello, world!"));

        assert_eq!(
            format!("{}", Subject::from("hello, world!")),
            String::from("hello, world!"),
            "Display implementation should format the subject as a plain string"
        );
    }

    #[test]
    fn test_from_str_creates_valid_subject() {
        let subject = String::from(Subject::from("hello, world!"));

        assert_eq!(
            subject,
            String::from("hello, world!"),
            "Subject created from &str should convert back to the original string"
        );
    }

    #[test]
    fn test_from_string_creates_valid_subject() {
        let subject = String::from(Subject::from(String::from("hello, world!")));

        assert_eq!(
            subject,
            String::from("hello, world!"),
            "Subject created from String should convert back to the original string"
        );
    }

    #[test]
    fn test_from_body_creates_equivalent_subject() {
        let subject = Subject::from(Body::from("hello, world!"));

        assert_eq!(
            subject,
            Subject::from("hello, world!"),
            "Subject created from Body should be equivalent to Subject created from the same string"
        );
    }

    #[test]
    fn test_from_fragments_extracts_first_body_as_subject() {
        let subject = Subject::from(vec![Fragment::Body(Body::from("hello, world!"))]);

        assert_eq!(
            subject,
            Subject::from("hello, world!"),
            "Subject created from fragments should extract the first Body fragment"
        );
    }

    #[test]
    fn test_from_cow_creates_valid_subject() {
        let subject = Subject::from(Cow::from("hello, world!"));

        assert_eq!(
            subject,
            Subject::from("hello, world!"),
            "Subject created from Cow should be equivalent to Subject created from the same string"
        );
    }

    #[test]
    fn test_from_fragments_skips_comments_when_extracting_subject() {
        let subject = Subject::from(vec![
            Fragment::Comment(Comment::from("# Important Comment")),
            Fragment::Body(Body::from("hello, world!")),
        ]);

        assert_eq!(
            subject,
            Subject::from("hello, world!"),
            "Subject created from fragments should skip Comment fragments and use the first Body fragment"
        );
    }
}
