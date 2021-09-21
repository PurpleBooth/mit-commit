const SCISSORS_MARKER: &str = "------------------------ >8 ------------------------";

/// The `Scissors` from a `CommitMessage`
#[derive(Debug, PartialEq, Clone)]
pub struct Scissors {
    scissors: String,
}

impl Scissors {
    pub(crate) fn parse_sections(message: &str) -> (String, Option<Scissors>) {
        if let Some(scissors_position) = message
            .lines()
            .position(|line| line.ends_with(SCISSORS_MARKER))
        {
            let lines = message.lines().collect::<Vec<_>>();
            let body = lines
                .clone()
                .into_iter()
                .take(scissors_position)
                .collect::<Vec<_>>()
                .join("\n");
            let scissors_string = &lines
                .into_iter()
                .skip(scissors_position)
                .collect::<Vec<_>>()
                .join("\n");

            let scissors = if let Some('\n') = message.chars().last() {
                Scissors::from(format!("{}\n", scissors_string))
            } else {
                Scissors::from(scissors_string)
            };

            (body, Some(scissors))
        } else {
            (message.to_string(), None)
        }
    }
}

impl From<&str> for Scissors {
    fn from(scissors: &str) -> Self {
        Scissors {
            scissors: String::from(scissors),
        }
    }
}

impl From<String> for Scissors {
    fn from(scissors: String) -> Self {
        Scissors { scissors }
    }
}

impl From<&String> for Scissors {
    fn from(scissors: &String) -> Self {
        Scissors {
            scissors: scissors.clone(),
        }
    }
}

impl From<Scissors> for String {
    fn from(scissors: Scissors) -> Self {
        scissors.scissors
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::Scissors;

    #[test]
    fn can_give_me_it_as_string() {
        let message = String::from(Scissors::from("hello, world!"));

        assert_eq!(message, String::from("hello, world!"));
    }

    #[test]
    fn it_can_be_created_from_a_string() {
        let message = String::from(Scissors::from(String::from("hello, world!")));

        assert_eq!(message, String::from("hello, world!"));
    }

    #[test]
    fn it_can_extract_itself_from_commit() {
        let sections = Scissors::parse_sections(indoc!(
            "
            Some text

            # ------------------------ >8 ------------------------
            # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            sections,
            (
                String::from("Some text\n"),
                Some(Scissors::from(indoc!(
                    "
                    # ------------------------ >8 ------------------------
                    # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                    # Alles unterhalb von ihr wird ignoriert.
                    diff --git a/file b/file
                    "
                )))
            )
        );
    }

    #[test]
    fn it_can_extract_itself_from_commit_with_a_standard_commit() {
        let sections = Scissors::parse_sections(indoc!(
            "
            Some text

            \u{00A3} ------------------------ >8 ------------------------
            \u{00A3} \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            \u{00A3} Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file"
        ));

        assert_eq!(
            sections,
            (
                String::from("Some text\n"),
                Some(Scissors {
                    scissors: indoc!(
                        "
                        \u{00A3} ------------------------ >8 ------------------------
                        \u{00A3} \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                        \u{00A3} Alles unterhalb von ihr wird ignoriert.
                        diff --git a/file b/file"
                    )
                    .into()
                })
            )
        );
    }
}
