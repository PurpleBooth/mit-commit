use crate::body::Body;
use crate::fragment::Fragment;

/// The `Subject` from the `CommitMessage`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Subject {
    text: String,
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
    use crate::body::Body;
    use crate::fragment::Fragment;
    use crate::Comment;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_be_created_from_a_str() {
        let subject = String::from(Subject::from("hello, world!"));

        assert_eq!(subject, String::from("hello, world!"))
    }
    #[test]
    fn it_can_be_created_from_a_string() {
        let subject = String::from(Subject::from(String::from("hello, world!")));

        assert_eq!(subject, String::from("hello, world!"))
    }
    #[test]
    fn it_can_be_created_from_a_body() {
        let subject = Subject::from(Body::from("hello, world!"));

        assert_eq!(subject, Subject::from("hello, world!"))
    }

    #[test]
    fn it_can_be_created_from_fragments() {
        let subject = Subject::from(vec![Fragment::Body(Body::from("hello, world!"))]);

        assert_eq!(subject, Subject::from("hello, world!"))
    }

    #[test]
    fn it_can_be_created_from_fragments_commit_first_is_skipped() {
        let subject = Subject::from(vec![
            Fragment::Comment(Comment::from("# Important Comment")),
            Fragment::Body(Body::from("hello, world!")),
        ]);

        assert_eq!(subject, Subject::from("hello, world!"))
    }
}
