use std::{
    fmt,
    fmt::{Display, Formatter},
    str::Chars,
};

use crate::{body::Body, fragment::Fragment};

/// The [`Subject`] from the [`CommitMessage`]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Subject {
    text: String,
}

impl Subject {
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
    #[must_use]
    pub fn chars(&self) -> Chars<'_> {
        self.text.chars()
    }
}

impl From<&str> for Subject {
    fn from(subject: &str) -> Self {
        Self {
            text: subject.into(),
        }
    }
}

impl From<String> for Subject {
    fn from(subject: String) -> Self {
        Self { text: subject }
    }
}

impl From<Subject> for String {
    fn from(subject: Subject) -> Self {
        subject.text
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Body> for Subject {
    fn from(body: Body) -> Self {
        Self::from(String::from(body))
    }
}

impl From<Vec<Fragment>> for Subject {
    fn from(ast: Vec<Fragment>) -> Self {
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
