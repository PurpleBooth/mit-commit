use std::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
    slice::Iter,
};

use crate::{body::Body, fragment::Fragment, trailer::Trailer};

/// A collection of user input `CommitMessage` text
#[derive(Debug, PartialEq, Clone)]
pub struct Bodies {
    bodies: Vec<Body>,
}

impl Bodies {
    /// Get the first `Body` in this list of `Bodies`
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
    pub fn first(&self) -> Option<Body> {
        self.bodies.first().cloned()
    }

    /// Iterate over the `Body` in the `Bodies`
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body};
    /// let trailers = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    /// let mut iterator = trailers.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 1")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 2")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Body> {
        self.bodies.iter()
    }
}

impl Display for Bodies {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Vec<Body>> for Bodies {
    fn from(bodies: Vec<Body>) -> Self {
        Bodies { bodies }
    }
}

impl From<Bodies> for String {
    fn from(bodies: Bodies) -> Self {
        bodies
            .bodies
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl From<Vec<Fragment>> for Bodies {
    fn from(bodies: Vec<Fragment>) -> Self {
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
            .collect::<Vec<Body>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::Bodies;
    use crate::{body::Body, fragment::Fragment};

    #[test]
    fn implements_iterator() {
        let trailers = Bodies::from(vec![
            Body::from("Body 1"),
            Body::from("Body 2"),
            Body::from("Body 3"),
        ]);
        let mut iterator = trailers.iter();

        assert_eq!(iterator.next(), Some(&Body::from("Body 1")));
        assert_eq!(iterator.next(), Some(&Body::from("Body 2")));
        assert_eq!(iterator.next(), Some(&Body::from("Body 3")));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn it_can_give_me_it_as_a_string() {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        assert_eq!(
            String::from(bodies),
            String::from(indoc!(
                "
                Message Body

                Another Message Body"
            ))
        );
    }

    #[test]
    fn it_can_be_formatted() {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        assert_eq!(
            format!("{}", bodies),
            String::from(indoc!(
                "
                Message Body

                Another Message Body"
            ))
        );
    }

    #[test]
    fn get_first() {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        assert_eq!(bodies.first(), Some(Body::from("Message Body")));
    }

    #[test]
    fn it_can_parse_itself_from_an_ast() {
        let bodies = Bodies::from(vec![
            Fragment::Body(Body::from("Subject Line")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Some content in the body of the message")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from(indoc!(
                "
                Co-authored-by: Billie Thomposon <billie@example.com>
                Co-authored-by: Someone Else <someone@example.com>
                "
            ))),
        ]);

        assert_eq!(
            bodies,
            Bodies::from(vec![
                Body::default(),
                Body::from("Some content in the body of the message"),
            ])
        );
    }
}
