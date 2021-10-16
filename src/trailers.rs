use std::{convert::TryFrom, slice::Iter};

use crate::{fragment::Fragment, trailer::Trailer};

/// A Collection of `Trailer`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Trailers {
    trailers: Vec<Trailer>,
    iterator_index: usize,
}

impl Trailers {
    /// Iterate over the [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
    ///     Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    ///     Trailer::new("Relates-to", "#124"),
    /// ]);
    /// let mut iterator = trailers.iter();
    ///
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by",
    ///         "Billie Thompson <billie@example.com>"
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by",
    ///         "Someone Else <someone@example.com>"
    ///     ))
    /// );
    /// assert_eq!(iterator.next(), Some(&Trailer::new("Relates-to", "#124")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Trailer> {
        self.trailers.iter()
    }

    /// How many [`Trailers`] are there
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
    ///     Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    /// ]);
    ///
    /// assert_eq!(trailers.len(), 2)
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.trailers.len()
    }

    /// Are there no [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// assert_eq!(
    ///     Trailers::from(vec![
    ///         Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
    ///         Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    ///     ])
    ///     .is_empty(),
    ///     false
    /// );
    ///
    /// let trailers: Vec<Trailer> = Vec::new();
    /// assert_eq!(Trailers::from(trailers).is_empty(), true)
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.trailers.is_empty()
    }
}

impl IntoIterator for Trailers {
    type IntoIter = std::vec::IntoIter<Trailer>;
    type Item = Trailer;

    /// Iterate over the [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
    ///     Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
    ///     Trailer::new("Relates-to", "#124"),
    /// ]);
    /// let mut iterator = trailers.into_iter();
    ///
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(Trailer::new(
    ///         "Co-authored-by",
    ///         "Billie Thompson <billie@example.com>"
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(Trailer::new(
    ///         "Co-authored-by",
    ///         "Someone Else <someone@example.com>"
    ///     ))
    /// );
    /// assert_eq!(iterator.next(), Some(Trailer::new("Relates-to", "#124")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.trailers.into_iter()
    }
}

impl From<Vec<Trailer>> for Trailers {
    fn from(trailers: Vec<Trailer>) -> Self {
        Self {
            trailers,
            iterator_index: 0,
        }
    }
}

impl From<Trailers> for String {
    fn from(trailers: Trailers) -> Self {
        trailers
            .trailers
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<Vec<Fragment>> for Trailers {
    fn from(ast: Vec<Fragment>) -> Self {
        ast.into_iter()
            .filter_map(|values| {
                if let Fragment::Body(body) = values {
                    Some(body)
                } else {
                    None
                }
            })
            .rev()
            .filter_map(|body| {
                if body.is_empty() {
                    None
                } else {
                    Some(Trailer::try_from(body))
                }
            })
            .take_while(std::result::Result::is_ok)
            .flatten()
            .collect::<Vec<Trailer>>()
            .into_iter()
            .rev()
            .collect::<Vec<Trailer>>()
            .into()
    }
}
