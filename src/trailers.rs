use std::{convert::TryFrom, slice::Iter};

use crate::{fragment::Fragment, trailer::Trailer};

/// A Collection of `Trailer`
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Trailers<'a> {
    trailers: Vec<Trailer<'a>>,
    iterator_index: usize,
}

impl<'a> Trailers<'a> {
    /// Iterate over the [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into(),
    ///     ),
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into(),
    ///     ),
    ///     Trailer::new("Relates-to".into(), "#124".into()),
    /// ]);
    /// let mut iterator = trailers.iter();
    ///
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new("Relates-to".into(), "#124".into()))
    /// );
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, Trailer<'_>> {
        self.trailers.iter()
    }

    /// How many [`Trailers`] are there
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into(),
    ///     ),
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into(),
    ///     ),
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
    ///         Trailer::new(
    ///             "Co-authored-by".into(),
    ///             "Billie Thompson <billie@example.com>".into()
    ///         ),
    ///         Trailer::new(
    ///             "Co-authored-by".into(),
    ///             "Someone Else <someone@example.com>".into()
    ///         ),
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

impl<'a> IntoIterator for Trailers<'a> {
    type IntoIter = std::vec::IntoIter<Trailer<'a>>;
    type Item = Trailer<'a>;

    /// Iterate over the [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into(),
    ///     ),
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into(),
    ///     ),
    ///     Trailer::new("Relates-to".into(), "#124".into()),
    /// ]);
    /// let mut iterator = trailers.into_iter();
    ///
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(Trailer::new("Relates-to".into(), "#124".into()))
    /// );
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.trailers.into_iter()
    }
}
impl<'a> IntoIterator for &'a Trailers<'a> {
    type IntoIter = Iter<'a, Trailer<'a>>;
    type Item = &'a Trailer<'a>;

    /// Iterate over the [`Trailers`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Borrow;
    ///
    /// use mit_commit::{Trailer, Trailers};
    /// let trailers = Trailers::from(vec![
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into(),
    ///     ),
    ///     Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into(),
    ///     ),
    ///     Trailer::new("Relates-to".into(), "#124".into()),
    /// ]);
    /// let trailer_ref = trailers.borrow();
    /// let mut iterator = trailer_ref.into_iter();
    ///
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Billie Thompson <billie@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new(
    ///         "Co-authored-by".into(),
    ///         "Someone Else <someone@example.com>".into()
    ///     ))
    /// );
    /// assert_eq!(
    ///     iterator.next(),
    ///     Some(&Trailer::new("Relates-to".into(), "#124".into()))
    /// );
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.trailers.iter()
    }
}

impl<'a> From<Vec<Trailer<'a>>> for Trailers<'a> {
    fn from(trailers: Vec<Trailer<'a>>) -> Self {
        Self {
            trailers,
            iterator_index: 0,
        }
    }
}

impl<'a> From<Trailers<'a>> for String {
    fn from(trailers: Trailers<'a>) -> Self {
        trailers
            .trailers
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<'a> From<Vec<Fragment<'a>>> for Trailers<'a> {
    fn from(ast: Vec<Fragment<'a>>) -> Self {
        ast.into_iter()
            .skip(1)
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
            .collect::<Vec<Trailer<'_>>>()
            .into_iter()
            .rev()
            .collect::<Vec<Trailer<'_>>>()
            .into()
    }
}
