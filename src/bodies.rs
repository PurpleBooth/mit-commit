use std::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
    slice::Iter,
    vec::IntoIter,
};

use crate::{body::Body, fragment::Fragment, trailer::Trailer};

/// A collection of body paragraphs from a commit message.
///
/// This struct represents multiple body paragraphs that make up the content of a commit message.
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
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Bodies<'a> {
    bodies: Vec<Body<'a>>,
}

impl Bodies<'_> {
    /// Get the first [`Body`] in this list of [`Bodies`]
    ///
    /// # Arguments
    ///
    /// * `self` - The Bodies collection to get the first element from
    ///
    /// # Returns
    ///
    /// The first Body in the list, or None if the list is empty
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
    /// # Arguments
    ///
    /// * `self` - The Bodies collection to iterate over
    ///
    /// # Returns
    ///
    /// An iterator over the Body elements in the Bodies collection
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
    pub fn iter(&self) -> Iter<'_, Body<'_>> {
        self.bodies.iter()
    }
}

impl<'a> IntoIterator for Bodies<'a> {
    type Item = Body<'a>;
    type IntoIter = IntoIter<Body<'a>>;

    /// Iterate over the [`Body`] in the [`Bodies`]
    ///
    /// # Arguments
    ///
    /// * `self` - The Bodies collection to consume and iterate over
    ///
    /// # Returns
    ///
    /// An iterator that takes ownership of the Bodies collection
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

impl<'a> IntoIterator for &'a Bodies<'a> {
    type IntoIter = Iter<'a, Body<'a>>;
    type Item = &'a Body<'a>;

    /// Iterate over the [`Body`] in the [`Bodies`]
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the Bodies collection to iterate over
    ///
    /// # Returns
    ///
    /// An iterator over references to the Body elements in the Bodies collection
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Borrow;
    ///
    /// use mit_commit::{Bodies, Body};
    /// let bodies = Bodies::from(vec![
    ///     Body::from("Body 1"),
    ///     Body::from("Body 2"),
    ///     Body::from("Body 3"),
    /// ]);
    /// let bodies_ref = bodies.borrow();
    /// let mut iterator = bodies_ref.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 1")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 2")));
    /// assert_eq!(iterator.next(), Some(&Body::from("Body 3")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.bodies.iter()
    }
}

impl Display for Bodies<'_> {
    /// Render the [`Bodies`] as text
    ///
    /// # Arguments
    ///
    /// * `self` - The Bodies collection to format
    /// * `f` - The formatter to write the formatted string to
    ///
    /// # Returns
    ///
    /// A string representation of the Bodies with each Body separated by double newlines
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
        if self.bodies.is_empty() {
            return Ok(());
        }

        let mut iter = self.bodies.iter();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
            for body in iter {
                write!(f, "\n\n{body}")?;
            }
        }

        Ok(())
    }
}

impl<'a> From<Vec<Body<'a>>> for Bodies<'a> {
    /// Combine a [`Vec`] of [`Body`] into [`Bodies`]
    ///
    /// # Arguments
    ///
    /// * `bodies` - A vector of Body objects to be combined into a Bodies collection
    ///
    /// # Returns
    ///
    /// A new Bodies instance containing all the provided Body objects
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

impl From<Bodies<'_>> for String {
    /// Convert a [`Bodies`] collection to a [`String`]
    ///
    /// # Arguments
    ///
    /// * `bodies` - The Bodies collection to convert to a string
    ///
    /// # Returns
    ///
    /// A string representation of the Bodies with each Body separated by double newlines
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
    /// assert_eq!(String::from(bodies), "Body 1\n\nBody 2\n\nBody 3");
    /// ```
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
    /// Convert a vector of [`Fragment`] to [`Bodies`]
    ///
    /// This extracts all Body fragments from the input, skipping the first one (which is typically
    /// the subject line) and any trailers at the end of the message.
    ///
    /// # Arguments
    ///
    /// * `bodies` - A vector of Fragment objects to extract Body fragments from
    ///
    /// # Returns
    ///
    /// A new Bodies instance containing only the Body fragments that are not the subject line
    /// and not trailers
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Bodies, Body, Fragment};
    ///
    /// let fragments = vec![
    ///     Fragment::Body(Body::from("Subject Line")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Body(Body::from("Some content in the body")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Body(Body::from("Co-authored-by: Someone <someone@example.com>")),
    /// ];
    ///
    /// let bodies = Bodies::from(fragments);
    ///
    /// assert_eq!(
    ///     bodies,
    ///     Bodies::from(vec![
    ///         Body::default(),
    ///         Body::from("Some content in the body"),
    ///     ])
    /// );
    /// ```
    fn from(bodies: Vec<Fragment<'a>>) -> Self {
        // Extract all Body fragments
        let raw_body = bodies
            .iter()
            .filter_map(|fragment| match fragment {
                Fragment::Body(body) => Some(body.clone()),
                Fragment::Comment(_) => None,
            })
            .collect::<Vec<_>>();

        // Count trailers at the end (including empty lines before them)
        let trailer_count = raw_body
            .iter()
            .skip(1)
            .rev()
            .take_while(|body| body.is_empty() || Trailer::try_from((*body).clone()).is_ok())
            .count();

        // Calculate how many non-trailer items to keep, excluding the subject line
        let non_trailer_item_count = raw_body
            .len()
            .saturating_sub(trailer_count)
            .saturating_sub(1);

        // Extract the body content, skipping subject and trailers
        raw_body
            .into_iter()
            .skip(1) // Skip subject line
            .take(non_trailer_item_count) // Take only non-trailer content
            .collect::<Vec<Body<'_>>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::Bodies;
    use crate::{body::Body, fragment::Fragment};

    #[test]
    fn test_iter_returns_bodies_in_order() -> Result<(), String> {
        let bodies = Bodies::from(vec![
            Body::from("Body 1"),
            Body::from("Body 2"),
            Body::from("Body 3"),
        ]);
        let mut iterator = bodies.iter();

        if iterator.next() != Some(&Body::from("Body 1")) {
            return Err("First body should be 'Body 1'".to_string());
        }

        if iterator.next() != Some(&Body::from("Body 2")) {
            return Err("Second body should be 'Body 2'".to_string());
        }

        if iterator.next() != Some(&Body::from("Body 3")) {
            return Err("Third body should be 'Body 3'".to_string());
        }

        if iterator.next().is_some() {
            return Err("Iterator should be exhausted after three elements".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_from_bodies_to_string_conversion_formats_correctly() -> Result<(), String> {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        let expected = String::from(indoc!(
            "
            Message Body

            Another Message Body"
        ));

        if String::from(bodies) != expected {
            return Err(
                "Bodies should be converted to a string with double newlines between them"
                    .to_string(),
            );
        }

        Ok(())
    }

    #[test]
    fn test_display_trait_formats_bodies_correctly() -> Result<(), String> {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        let expected = String::from(indoc!(
            "
            Message Body

            Another Message Body"
        ));

        if format!("{bodies}") != expected {
            return Err(
                "Display implementation should format bodies with double newlines between them"
                    .to_string(),
            );
        }

        Ok(())
    }

    #[test]
    fn test_first_returns_first_body_when_present() -> Result<(), String> {
        let bodies = Bodies::from(vec![
            Body::from("Message Body"),
            Body::from("Another Message Body"),
        ]);

        if bodies.first() != Some(Body::from("Message Body")) {
            return Err("First method should return the first body in the collection".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_from_fragments_extracts_body_content_correctly() {
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
            ]),
            "From<Vec<Fragment>> should extract body content, skipping subject and trailers"
        );
    }
}
