const SCISSORS_MARKER: &str = "------------------------ >8 ------------------------";

/// The [`Scissors`] from a [`CommitMessage`]
#[derive(Debug, PartialEq, Clone)]
pub struct Scissors {
    scissors: String,
}

impl Scissors {
    pub(crate) fn parse_sections(message: &str) -> (String, Option<Self>) {
        message
            .lines()
            .position(|line| line.ends_with(SCISSORS_MARKER))
            .map_or_else(
                || (message.to_string(), None),
                |scissors_position| {
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

                    let scissors = if message.ends_with('\n') {
                        Self::from(format!("{}\n", scissors_string))
                    } else {
                        Self::from(scissors_string)
                    };

                    (body, Some(scissors))
                },
            )
    }
}

impl From<&str> for Scissors {
    fn from(scissors: &str) -> Self {
        Self {
            scissors: String::from(scissors),
        }
    }
}

impl From<String> for Scissors {
    fn from(scissors: String) -> Self {
        Self { scissors }
    }
}

impl From<&String> for Scissors {
    fn from(scissors: &String) -> Self {
        Self {
            scissors: scissors.clone(),
        }
    }
}

impl From<Scissors> for String {
    fn from(scissors: Scissors) -> Self {
        scissors.scissors
    }
}
