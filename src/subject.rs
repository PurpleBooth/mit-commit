use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
    str::Chars,
};

use crate::{body::Body, fragment::Fragment};

/// The [`Subject`] from the [`CommitMessage`]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Subject<'a> {
    text: Cow<'a, str>,
}

impl Subject<'_> {
    /// Count characters in [`Subject`]
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

    /// Is the [`Subject`] empty
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

    /// Convert the [`Subject`] into chars
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
