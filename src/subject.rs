use std::{
    fmt,
    fmt::{Display, Formatter},
    str::Chars,
};

use crate::{body::Body, fragment::Fragment};

/// The `Subject` from the `CommitMessage`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Subject {
    text: String,
}

impl Subject {
    /// Count characters in `Subject`
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

    /// Is the `Subject` empty
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

    /// Convert the `Subject` into chars
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
        Subject {
            text: subject.into(),
        }
    }
}

impl From<String> for Subject {
    fn from(subject: String) -> Self {
        Subject { text: subject }
    }
}

impl From<Subject> for String {
    fn from(subject: Subject) -> String {
        subject.text
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Body> for Subject {
    fn from(body: Body) -> Subject {
        Subject::from(String::from(body))
    }
}

impl From<Vec<Fragment>> for Subject {
    fn from(ast: Vec<Fragment>) -> Self {
        ast.iter()
            .find_map(|values| {
                if let Fragment::Body(body) = values {
                    Some(Subject::from(body.clone()))
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {

    use super::Subject;
    use crate::{body::Body, fragment::Fragment, Comment};

    #[test]
    fn len() {
        assert_eq!(Subject::from("hello, world!").len(), 13);
        assert_eq!(Subject::from("goodbye").len(), 7);
    }

    #[test]
    fn chars() {
        let subject = Subject::from("y\u{306}");

        let mut chars = subject.chars();

        assert_eq!(Some('y'), chars.next());
        assert_eq!(Some('\u{0306}'), chars.next());

        assert_eq!(None, chars.next());
    }

    #[test]
    fn is_empty() {
        assert!(!Subject::from("hello, world!").is_empty());
        assert!(Subject::from("").is_empty());
    }

    #[test]
    fn it_can_be_formatted() {
        let _subject = String::from(Subject::from("hello, world!"));

        assert_eq!(
            format!("{}", Subject::from("hello, world!")),
            String::from("hello, world!")
        );
    }

    #[test]
    fn it_can_be_created_from_a_str() {
        let subject = String::from(Subject::from("hello, world!"));

        assert_eq!(subject, String::from("hello, world!"));
    }

    #[test]
    fn it_can_be_created_from_a_string() {
        let subject = String::from(Subject::from(String::from("hello, world!")));

        assert_eq!(subject, String::from("hello, world!"));
    }

    #[test]
    fn it_can_be_created_from_a_body() {
        let subject = Subject::from(Body::from("hello, world!"));

        assert_eq!(subject, Subject::from("hello, world!"));
    }

    #[test]
    fn it_can_be_created_from_fragments() {
        let subject = Subject::from(vec![Fragment::Body(Body::from("hello, world!"))]);

        assert_eq!(subject, Subject::from("hello, world!"));
    }

    #[test]
    fn it_can_be_created_from_fragments_commit_first_is_skipped() {
        let subject = Subject::from(vec![
            Fragment::Comment(Comment::from("# Important Comment")),
            Fragment::Body(Body::from("hello, world!")),
        ]);

        assert_eq!(subject, Subject::from("hello, world!"));
    }
}
