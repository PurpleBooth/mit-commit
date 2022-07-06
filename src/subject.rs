use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
    str::Chars,
};

use nom::{
    branch::alt,
    bytes::complete::{take, take_until1},
    character::complete::char,
    combinator::{map, not, peek, recognize, rest},
    sequence::{pair, tuple},
    IResult,
};

use crate::{body::Body, fragment::Fragment};

/// The [`Subject`] from the [`CommitMessage`]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Subject<'a> {
    text: Cow<'a, str>,
}

impl<'a> Subject<'a> {
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

    /// Create a parser for parsing the subject
    pub fn parser<E: nom::error::ParseError<&'a str> + 'a>(
        comment_char: char,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Subject<'a>, E> + 'a
    where
        E: 'a,
    {
        map(
            recognize(tuple((
                peek(not(char::<&'a str, _>(comment_char))),
                alt((recognize(pair(take_until1("\n\n"), take(2_usize))), rest)),
            ))),
            |raw_subject: &'a str| -> Subject<'a> { Cow::from(raw_subject).into() },
        )
    }
}

impl<'a> From<&'a str> for Subject<'a> {
    fn from(subject: &'a str) -> Self {
        Self {
            text: subject.into(),
        }
    }
}

impl<'a> From<String> for Subject<'a> {
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

impl<'a> From<Subject<'a>> for String {
    fn from(subject: Subject<'_>) -> Self {
        subject.text.into_owned()
    }
}

impl<'a> Display for Subject<'a> {
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
