use crate::fragment::Fragment;
use crate::trailer::Trailer;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct Trailers {
    trailers: Vec<Trailer>,
}

impl From<Vec<Trailer>> for Trailers {
    fn from(trailers: Vec<Trailer>) -> Self {
        Trailers { trailers }
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
    use super::Trailers;
    use crate::fragment::Fragment;
    use crate::trailer::Trailer;
    use crate::Body;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

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
