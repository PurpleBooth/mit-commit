use std::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
    slice::Iter,
    vec::IntoIter,
};

use crate::{body::Body, fragment::Fragment, trailer::Trailer};

/// A collection of user input [`CommitMessage`] text
///
/// # Examples
///
/// ```
/// use mit_commit::{Bodies, Body, Subject};
///
/// let bodies: Vec<Body> = Vec::default();
/// assert_eq!(None, Bodies::from(bodies).first());
///
/// let bodies: Vec<Body> = vec![
///     Body::from("First"),
///     Body::from("Second"),
///     Body::from("Third"),
/// ];
/// assert_eq!(Some(Body::from("First")), Bodies::from(bodies).first());
/// ```
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Bodies<'a> {
    bodies: Vec<Body<'a>>,
}

impl<'a> Bodies<'a> {
    /// Get the first [`Body`] in this list of [`Bodies`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body, Subject};
    ///
    /// let bodies: Vec<Body> = Vec::default();
    /// assert_eq!(None, Bodies::from(bodies).first());
    ///
    /// let bodies: Vec<Body> = vec![
    ///     Body::from("First"),
    ///     Body::from("Second"),
    ///     Body::from("Third"),
    /// ];
    /// assert_eq!(Some(Body::from("First")), Bodies::from(bodies).first());
    /// ```
    #[must_use]
    pub fn first(&self) -> Option<Body<'_>> {
        self.bodies.first().cloned()
    }

    /// Iterate over the [`Body`] in the [`Bodies`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body};
    /// let bodies = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    /// let mut iterator = bodies.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 1")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 2")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Body<'_>> {
        self.bodies.iter()
    }
}

impl<'a> IntoIterator for Bodies<'a> {
    type IntoIter = IntoIter<Body<'a>>;
    type Item = Body<'a>;

    /// Iterate over the [`Body`] in the [`Bodies`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body};
    /// let bodies = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    /// let mut iterator = bodies.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(Body::from("Body 1")));
    /// assert_eq!(iterator.next(), Some(Body::from("Body 2")));
    /// assert_eq!(iterator.next(), Some(Body::from("Body 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.bodies.into_iter()
    }
}

impl<'a> Display for Bodies<'a> {
    /// Render the [`Bodies`] as text
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body};
    /// let bodies = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    ///
    /// assert_eq!(format!("{}", bodies), "Body 1\n\nBody 2\n\nBody 3");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl<'a> From<Vec<Body<'a>>> for Bodies<'a> {
    /// Combine a [`Vec`] of [`Body`] into [`Bodies`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body};
    /// let bodies = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    /// let mut iterator = bodies.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(Body::from("Body 1")));
    /// assert_eq!(iterator.next(), Some(Body::from("Body 2")));
    /// assert_eq!(iterator.next(), Some(Body::from("Body 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn from(bodies: Vec<Body<'a>>) -> Self {
        Self { bodies }
    }
}

impl<'a> From<Bodies<'a>> for String {
    fn from(bodies: Bodies<'_>) -> Self {
        bodies
            .bodies
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl<'a> From<Vec<Fragment<'a>>> for Bodies<'a> {
    fn from(bodies: Vec<Fragment<'a>>) -> Self {
        let raw_body = bodies
            .iter()
            .filter_map(|values| {
                if let Fragment::Body(body) = values {
                    Some(body.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let trailer_count = raw_body
            .clone()
            .into_iter()
            .rev()
            .take_while(|body| body.is_empty() || Trailer::try_from(body.clone()).is_ok())
            .count();
        let mut non_trailer_item_count = raw_body.len() - trailer_count;
        non_trailer_item_count = non_trailer_item_count.saturating_sub(1);

        raw_body
            .into_iter()
            .enumerate()
            .skip(1)
            .take(non_trailer_item_count)
            .map(|(_, body)| body)
            .collect::<Vec<Body<'_>>>()
            .into()
    }
}
