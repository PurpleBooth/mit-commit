use std::borrow::Cow;

use crate::Comment;

const SCISSORS_MARKER: &str = "------------------------ >8 ------------------------";

/// The [`Scissors`] from a [`CommitMessage`]
#[derive(Debug, PartialEq, Clone)]
pub struct Scissors<'a> {
    scissors: Cow<'a, str>,
}

impl<'a> Scissors<'a> {
    pub(crate) fn guess_comment_character(message: &str) -> Option<char> {
        if let Some(scissors_guess) = Self::guess_comment_char_from_scissors(message) {
            return Some(scissors_guess);
        }

        Self::guess_comment_char_from_last_possibility(message)
    }

    fn guess_comment_char_from_last_possibility(message: &str) -> Option<char> {
        message
            .lines()
            .filter_map(|line| line.chars().next())
            .filter(|first_letter| Comment::is_legal_comment_char(*first_letter))
            .last()
    }

    fn guess_comment_char_from_scissors(message: &str) -> Option<char> {
        message
            .lines()
            .filter(|line| match line.chars().next() {
                None => false,
                Some(first_letter) => Comment::is_legal_comment_char(first_letter),
            })
            .filter(|line| line.chars().count() == SCISSORS_MARKER.chars().count() + 2)
            .filter(|line| {
                line.chars()
                    .nth(1)
                    .filter(|second_letter| *second_letter == ' ')
                    .is_some()
            })
            .filter(|line| line.ends_with(SCISSORS_MARKER))
            .filter_map(|line| line.chars().next())
            .last()
    }

    pub(crate) fn parse_sections(message: &str) -> (Cow<'a, str>, Option<Scissors<'a>>) {
        message
            .lines()
            .position(|line| line.ends_with(SCISSORS_MARKER))
            .map_or_else(
                || (message.to_string().into(), None),
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
                        Self::from(scissors_string.clone())
                    };

                    (body.into(), Some(scissors))
                },
            )
    }
}

impl<'a> From<Cow<'a, str>> for Scissors<'a> {
    fn from(scissors: Cow<'a, str>) -> Self {
        Self { scissors }
    }
}

impl<'a> From<&'a str> for Scissors<'a> {
    fn from(scissors: &'a str) -> Self {
        Self {
            scissors: scissors.into(),
        }
    }
}

impl<'a> From<String> for Scissors<'a> {
    fn from(scissors: String) -> Self {
        Self {
            scissors: scissors.into(),
        }
    }
}

impl<'a> From<Scissors<'a>> for String {
    fn from(scissors: Scissors<'a>) -> Self {
        scissors.scissors.into()
    }
}
