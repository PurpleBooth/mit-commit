use std::convert::TryFrom;
use std::slice::Iter;

use crate::fragment::Fragment;
use crate::trailer::Trailer;

/// A Collection of `Trailer`
#[derive(Debug, PartialEq, Clone)]
pub struct Trailers {
    trailers: Vec<Trailer>,
    iterator_index: usize,
}

impl Trailers {
    /// Iterate over the trailers
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Trailer;
    /// use mit_commit::Trailers;
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

    /// How many trailers are there
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Trailer;
    /// use mit_commit::Trailers;
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

    /// Are there no trailers
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::Trailer;
    /// use mit_commit::Trailers;
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

impl From<Vec<Trailer>> for Trailers {
    fn from(trailers: Vec<Trailer>) -> Self {
        Trailers {
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
            .map(String::from)
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use indoc::indoc;

    use crate::fragment::Fragment;
    use crate::trailer::Trailer;
    use crate::Body;

    use super::Trailers;

    #[test]
    fn implements_iterator() {
        let trailers = Trailers::from(vec![
            Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
            Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
            Trailer::new("Relates-to", "#124"),
        ]);
        let mut iterator = trailers.iter();

        assert_eq!(
            iterator.next(),
            Some(&Trailer::new(
                "Co-authored-by",
                "Billie Thompson <billie@example.com>",
            ))
        );
        assert_eq!(
            iterator.next(),
            Some(&Trailer::new(
                "Co-authored-by",
                "Someone Else <someone@example.com>",
            ))
        );
        assert_eq!(iterator.next(), Some(&Trailer::new("Relates-to", "#124")));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn it_can_give_me_it_as_a_string() {
        let trailers = Trailers::from(vec![Trailer::new(
            "Co-authored-by",
            "Billie Thompson <billie@example.com>",
        )]);

        assert_eq!(
            String::from(trailers),
            String::from("Co-authored-by: Billie Thompson <billie@example.com>")
        )
    }

    #[test]
    fn it_can_give_me_the_length() {
        let trailers = Trailers::from(vec![
            Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
            Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
        ]);

        assert_eq!(trailers.len(), 2)
    }

    #[test]
    fn it_can_tell_me_if_it_is_empty() {
        assert_eq!(
            Trailers::from(vec![
                Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
                Trailer::new("Co-authored-by", "Someone Else <someone@example.com>"),
            ])
            .is_empty(),
            false
        );

        let trailers: Vec<Trailer> = Vec::new();
        assert_eq!(Trailers::from(trailers).is_empty(), true)
    }

    #[test]
    fn it_can_be_constructed_from_ast() {
        let trailers = vec![
            Fragment::Body(Body::from("Example Commit")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from(indoc!(
                "
                This is an example commit. This is to illustrate something for a test and would be
                pretty unusual to find in an actual git history.
                "
            ))),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from(
                "Co-authored-by: Billie Thompson <billie@example.com>",
            )),
            Fragment::Body(Body::from(
                "Co-authored-by: Somebody Else <somebody@example.com>",
            )),
        ];

        let expected: Trailers = vec![
            Trailer::new("Co-authored-by", "Billie Thompson <billie@example.com>"),
            Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
        ]
        .into();

        assert_eq!(Trailers::from(trailers), expected)
    }
}
