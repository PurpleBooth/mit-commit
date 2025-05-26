use std::borrow::Cow;

use crate::Comment;

const SCISSORS_MARKER: &str = "------------------------ >8 ------------------------";

/// The [`Scissors`] from a [`CommitMessage`]
///
/// Represents the scissors section of a commit message, which separates the commit message
/// from the diff or other content that should not be included in the commit message.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scissors<'a> {
    scissors: Cow<'a, str>,
}

impl<'a> Scissors<'a> {
    /// Attempts to guess the comment character used in a commit message.
    ///
    /// # Arguments
    ///
    /// * `message` - The commit message to analyze
    ///
    /// # Returns
    ///
    /// The comment character if one can be determined, or None if no comment character is found
    pub(crate) fn guess_comment_character(message: &str) -> Option<char> {
        Self::guess_comment_char_from_scissors(message)
            .or_else(|| Self::guess_comment_char_from_last_possibility(message))
    }

    /// Attempts to guess the comment character by looking at the first character of each line.
    ///
    /// # Arguments
    ///
    /// * `message` - The commit message to analyze
    ///
    /// # Returns
    ///
    /// The last valid comment character found, or None if no valid comment character is found
    fn guess_comment_char_from_last_possibility(message: &str) -> Option<char> {
        message
            .lines()
            .filter_map(|line| {
                line.chars()
                    .next()
                    .filter(|first_letter| Comment::is_legal_comment_char(*first_letter))
            })
            .next_back()
    }

    /// Attempts to guess the comment character by looking for scissors markers.
    ///
    /// # Arguments
    ///
    /// * `message` - The commit message to analyze
    ///
    /// # Returns
    ///
    /// The comment character from the scissors line, or None if no scissors line is found
    fn guess_comment_char_from_scissors(message: &str) -> Option<char> {
        message
            .lines()
            .filter_map(|line| {
                let mut line_chars = line.chars();
                let first_character = line_chars.next();
                first_character.filter(|cc| Comment::is_legal_comment_char(*cc))?;
                line_chars.next().filter(|cc| *cc == ' ')?;

                if SCISSORS_MARKER != line_chars.as_str() {
                    return None;
                }

                first_character
            })
            .next_back()
    }

    /// Parses a commit message into body and scissors sections.
    ///
    /// # Arguments
    ///
    /// * `message` - The commit message to parse
    ///
    /// # Returns
    ///
    /// A tuple containing the body of the commit message and an optional scissors section
    pub(crate) fn parse_sections(message: &str) -> (Cow<'a, str>, Option<Self>) {
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

            let scissors = if message.ends_with('\n') {
                Self::from(format!("{scissors_string}\n"))
            } else {
                Self::from(scissors_string.clone())
            };

            (body.into(), Some(scissors))
        } else {
            // No scissors section found
            (message.to_string().into(), None)
        }
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

impl From<String> for Scissors<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn can_give_me_it_as_string() {
        let message = String::from(Scissors::from("hello, world!"));

        assert_eq!(
            message,
            String::from("hello, world!"),
            "Converting Scissors to String should preserve the content"
        );
    }

    #[test]
    fn it_can_be_created_from_a_string() {
        let message = String::from(Scissors::from(String::from("hello, world!")));

        assert_eq!(
            message,
            String::from("hello, world!"),
            "Creating Scissors from String and converting back should preserve the content"
        );
    }

    #[test]
    fn it_can_guess_the_comment_character_from_scissors_without_other_parts() {
        let comment_char = Scissors::guess_comment_character(
            "# ------------------------ >8 ------------------------\n! Not the comment",
        );

        assert_eq!(
            comment_char,
            Some('#'),
            "Should identify '#' as the comment character from the scissors line"
        );
    }

    #[test]
    fn it_can_guess_the_comment_character_from_scissors_without_comment() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text

              ------------------------ >8 ------------------------
            ; ------------------------ >8 ------------------------
            ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            ; Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            comment_char,
            Some(';'),
            "Should identify ';' as the comment character from the scissors line"
        );
    }

    #[test]
    fn it_only_needs_the_scissors_and_no_there_lines() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text
            ; ------------------------ >8 ------------------------
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            comment_char,
            Some(';'),
            "Should identify ';' as the comment character from a single scissors line"
        );
    }

    #[test]
    fn it_checks_a_space_must_be_after_the_comment_character_for_scissors_comment_guess() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text

            ##------------------------ >8 ------------------------
            ; ------------------------ >8 ------------------------
            ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            ; Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            comment_char,
            Some(';'),
            "Should require a space after the comment character in scissors line"
        );
    }

    #[test]
    fn it_checks_there_are_no_additional_characters() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text

            # !!!!!!!------------------------ >8 ------------------------
            ; ------------------------ >8 ------------------------
            ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            ; Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            comment_char,
            Some(';'),
            "Should not recognize lines with additional characters between comment and scissors marker"
        );
    }

    #[test]
    fn it_takes_the_last_scissors_if_there_are_multiple() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text

            # ------------------------ >8 ------------------------
            ; ------------------------ >8 ------------------------
            ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
            ; Alles unterhalb von ihr wird ignoriert.
            diff --git a/file b/file
            "
        ));

        assert_eq!(
            comment_char,
            Some(';'),
            "Should use the last scissors line's comment character when multiple are present"
        );
    }

    #[test]
    fn it_returns_none_on_a_failure_to_find_the_comment_char_from_scissors() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            Some text
            "
        ));

        assert_eq!(
            comment_char, None,
            "Should return None when no scissors line is found"
        );
    }

    #[test]
    fn it_returns_none_on_empty_string() {
        let comment_char = Scissors::guess_comment_character("");

        assert_eq!(comment_char, None, "Should return None for empty string");
    }

    #[test]
    fn it_returns_none_on_just_newlines() {
        let comment_char = Scissors::guess_comment_character(&"\n".repeat(5));

        assert_eq!(
            comment_char, None,
            "Should return None for string with only newlines"
        );
    }

    #[test]
    fn it_returns_the_last_valid_comment_when_there_are_multiple_options() {
        let comment_char = Scissors::guess_comment_character(indoc!(
            "
            # I am a potential comment
            @ I am a potential comment
            ? I am a potential comment
            "
        ));

        assert_eq!(
            comment_char,
            Some('@'),
            "Should return the last valid comment character when no scissors line is found"
        );
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
                Cow::from("Some text\n"),
                Some(Scissors::from(indoc!(
                    "
                    # ------------------------ >8 ------------------------
                    # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                    # Alles unterhalb von ihr wird ignoriert.
                    diff --git a/file b/file
                    "
                )))
            ),
            "Should correctly split the commit message at the scissors line"
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
                Cow::from("Some text\n"),
                Some(Scissors::from(indoc!(
                    "
                    \u{00A3} ------------------------ >8 ------------------------
                    \u{00A3} \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                    \u{00A3} Alles unterhalb von ihr wird ignoriert.
                    diff --git a/file b/file"
                )))
            ),
            "Should correctly split the commit message with non-ASCII comment characters"
        );
    }
}
