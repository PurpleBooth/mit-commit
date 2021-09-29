use std::{convert::TryFrom, fs::File, io, io::Read, path::PathBuf};

use miette::Diagnostic;
use regex::Regex;
use thiserror::Error;

use super::{
    bodies::Bodies,
    body::Body,
    comment::Comment,
    comments::Comments,
    fragment::Fragment,
    subject::Subject,
    trailers::Trailers,
};
use crate::{scissors::Scissors, Trailer};

/// A [`CommitMessage`], the primary entry point to the library
#[derive(Debug, PartialEq, Clone)]
pub struct CommitMessage {
    scissors: Option<Scissors>,
    ast: Vec<Fragment>,
    subject: Subject,
    trailers: Trailers,
    comments: Comments,
    bodies: Bodies,
}

impl CommitMessage {
    /// Convert from [`Fragment`] back into a full [`CommitMessage`]
    ///
    /// Get back to a [`CommitMessage`] from an ast, usually after you've been
    /// editing the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Bodies, CommitMessage, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Update bashrc to include kubernetes completions
    ///
    ///     This should make it easier to deploy things for the developers.
    ///     Benchmarked with Hyperfine, no noticable performance decrease.
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ));
    /// assert_eq!(
    ///     CommitMessage::from_fragments(message.get_ast(), message.get_scissors()),
    ///     message,
    /// )
    /// ```
    #[must_use]
    pub fn from_fragments(fragments: Vec<Fragment>, scissors: Option<Scissors>) -> CommitMessage {
        let body = fragments
            .into_iter()
            .map(|x| match x {
                Fragment::Body(contents) => String::from(contents),
                Fragment::Comment(contents) => String::from(contents),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let scissors: String = if let Some(contents) = scissors {
            format!("\n{}", String::from(contents))
        } else {
            "".into()
        };

        CommitMessage::from(format!("{}{}", body, scissors))
    }

    /// A helper method to let you insert [`Trailer`]
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{CommitMessage, Trailer};
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///
    ///     This is an example commit message for linting
    ///
    ///     Relates-to: #153
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Auf Branch main
    ///     ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     file
    ///     ;
    ///     "
    /// ));
    ///
    /// assert_eq!(
    ///     String::from(commit.add_trailer(Trailer::new(
    ///         "Co-authored-by",
    ///         "Test Trailer <test@example.com>"
    ///     ))),
    ///     String::from(CommitMessage::from(indoc!(
    ///         "
    ///         Example Commit Message
    ///
    ///         This is an example commit message for linting
    ///
    ///         Relates-to: #153
    ///         Co-authored-by: Test Trailer <test@example.com>
    ///
    ///         ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///         ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///         ; bricht den Commit ab.
    ///         ;
    ///         ; Auf Branch main
    ///         ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///         ;
    ///         ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///         ;    neue Datei:     file
    ///         ;
    ///         "
    ///     )))
    /// );
    /// ```
    #[must_use]
    pub fn add_trailer(&self, trailer: Trailer) -> Self {
        let mut fragments = Vec::new();

        if self.bodies.iter().all(Body::is_empty) && self.trailers.is_empty() {
            fragments.push(Fragment::Body(Body::default()));
        }

        if self.trailers.is_empty() {
            fragments.push(Fragment::Body(Body::default()));
        }

        fragments.push(trailer.into());

        self.insert_after_last_full_body(fragments)
    }

    /// Insert text in the place you're most likely to want it
    ///
    /// In the case you don't have any full [`Body`] in there, it inserts it at
    /// the top of the commit, in the [`Subject`] line.
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{Fragment, Body, CommitMessage, Comment};
    ///
    ///         let ast: Vec<Fragment> = vec![
    ///             Fragment::Body(Body::from("Add file")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("Relates-to: #128")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#"))
    ///         ];
    ///         let commit = CommitMessage::from_fragments(ast, None);
    ///
    ///         assert_eq!(commit.insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))]).get_ast(), vec![
    ///             Fragment::Body(Body::from("Add file")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Body(Body::from("Relates-to: #128\nRelates-to: #656")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
    ///             Fragment::Body(Body::default()),
    ///             Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#"))
    ///         ])
    /// ```
    #[must_use]
    pub fn insert_after_last_full_body(&self, fragment: Vec<Fragment>) -> CommitMessage {
        let position = self.ast.iter().rposition(|fragment| match fragment {
            Fragment::Body(body) => !body.is_empty(),
            Fragment::Comment(_) => false,
        });

        let (before, after): (Vec<_>, Vec<_>) = match position {
            Some(position) => self
                .ast
                .clone()
                .into_iter()
                .enumerate()
                .partition(|(index, _)| index <= &position),
            None => (vec![], self.ast.clone().into_iter().enumerate().collect()),
        };

        CommitMessage::from_fragments(
            [
                before.into_iter().map(|(_, x)| x).collect(),
                fragment,
                after.into_iter().map(|(_, x)| x).collect(),
            ]
            .concat(),
            self.get_scissors(),
        )
    }

    fn convert_to_per_line_ast(comment_character: Option<char>, rest: &str) -> Vec<Fragment> {
        rest.lines()
            .map(|line| match comment_character {
                Some(comment_character) => {
                    if line.starts_with(comment_character) {
                        Fragment::Comment(Comment::from(line))
                    } else {
                        Fragment::Body(Body::from(line))
                    }
                }
                None => Fragment::Body(Body::from(line)),
            })
            .collect()
    }

    fn group_ast(ungrouped_ast: Vec<Fragment>) -> Vec<Fragment> {
        ungrouped_ast
            .into_iter()
            .fold(vec![], |acc: Vec<Fragment>, new_fragment| {
                let mut previous_fragments = acc.clone();
                match (acc.last(), &new_fragment) {
                    (None, fragment) => {
                        previous_fragments.push(fragment.clone());
                        previous_fragments
                    }
                    (Some(Fragment::Comment(existing)), Fragment::Comment(new)) => {
                        previous_fragments.truncate(acc.len() - 1);
                        previous_fragments.push(Fragment::Comment(existing.append(new)));
                        previous_fragments
                    }
                    (Some(Fragment::Body(existing)), Fragment::Body(new)) => {
                        if new.is_empty() || existing.is_empty() {
                            previous_fragments.push(Fragment::Body(new.clone()));
                        } else {
                            previous_fragments.truncate(acc.len() - 1);
                            previous_fragments.push(Fragment::Body(existing.append(new)));
                        }
                        previous_fragments
                    }
                    (Some(Fragment::Body(_)), Fragment::Comment(new)) => {
                        previous_fragments.push(Fragment::Comment(new.clone()));
                        previous_fragments
                    }
                    (Some(Fragment::Comment(_)), Fragment::Body(new)) => {
                        previous_fragments.push(Fragment::Body(new.clone()));
                        previous_fragments
                    }
                }
            })
    }

    /// Get the [`Subject`] line from the [`CommitMessage`]
    ///
    /// It's possible to get this from the ast, but it's a bit of a faff, so
    /// this is a convenience method
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Bodies, CommitMessage, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Update bashrc to include kubernetes completions
    ///
    ///     This should make it easier to deploy things for the developers.
    ///     Benchmarked with Hyperfine, no noticable performance decrease.
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ));
    /// assert_eq!(
    ///     message.get_subject(),
    ///     Subject::from("Update bashrc to include kubernetes completions")
    /// )
    /// ```
    #[must_use]
    pub fn get_subject(&self) -> Subject {
        self.subject.clone()
    }

    /// Get the underlying data structure that represents the [`CommitMessage`]
    ///
    /// This is the underlying datastructure for the [`CommitMessage`]. You
    /// might want this to create a complicated linter, or modify the
    /// [`CommitMessage`] to your liking.
    ///
    /// Notice how it doesn't include the [`Scissors`] section.
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Body, CommitMessage, Fragment, Trailer, Trailers, Comment};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Add file
    ///
    ///     Looks-like-a-trailer: But isn't
    ///
    ///     This adds file primarily for demonstration purposes. It might not be
    ///     useful as an actual commit, but it's very useful as a example to use in
    ///     tests.
    ///
    ///     Relates-to: #128
    ///     Relates-to: #129
    ///
    ///     ; Short (50 chars or less) summary of changes
    ///     ;
    ///     ; More detailed explanatory text, if necessary.  Wrap it to
    ///     ; about 72 characters or so.  In some contexts, the first
    ///     ; line is treated as the subject of an email and the rest of
    ///     ; the text as the body.  The blank line separating the
    ///     ; summary from the body is critical (unless you omit the body
    ///     ; entirely); tools like rebase can get confused if you run
    ///     ; the two together.
    ///     ;
    ///     ; Further paragraphs come after blank lines.
    ///     ;
    ///     ;   - Bullet points are okay, too
    ///     ;
    ///     ;   - Typically a hyphen or asterisk is used for the bullet,
    ///     ;     preceded by a single space, with blank lines in
    ///     ;     between, but conventions vary here
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Auf Branch main
    ///     ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;   neue Datei:     file
    ///     ;
    ///     ; ------------------------ >8 ------------------------
    ///     ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    ///     ; Alles unterhalb von ihr wird ignoriert.
    ///     diff --git a/file b/file
    ///     new file mode 100644
    ///     index 0000000..e69de29
    ///     "
    /// ));
    /// let ast = vec![
    ///     Fragment::Body(Body::from("Add file")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Body(Body::from("Looks-like-a-trailer: But isn't")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Body(Body::from("Relates-to: #128\nRelates-to: #129")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Comment(Comment::from("; Short (50 chars or less) summary of changes\n;\n; More detailed explanatory text, if necessary.  Wrap it to\n; about 72 characters or so.  In some contexts, the first\n; line is treated as the subject of an email and the rest of\n; the text as the body.  The blank line separating the\n; summary from the body is critical (unless you omit the body\n; entirely); tools like rebase can get confused if you run\n; the two together.\n;\n; Further paragraphs come after blank lines.\n;\n;   - Bullet points are okay, too\n;\n;   - Typically a hyphen or asterisk is used for the bullet,\n;     preceded by a single space, with blank lines in\n;     between, but conventions vary here")),
    ///     Fragment::Body(Body::default()),
    ///     Fragment::Comment(Comment::from("; Bitte geben Sie eine Commit-Beschreibung für Ihre änderungen ein. Zeilen,\n; die mit \';\' beginnen, werden ignoriert, und eine leere Beschreibung\n; bricht den Commit ab.\n;\n; Auf Branch main\n; Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n;\n; Zum Commit vorgemerkte änderungen:\n;   neue Datei:     file\n;"))
    /// ];
    /// assert_eq!(message.get_ast(), ast)
    /// ```
    #[must_use]
    pub fn get_ast(&self) -> Vec<Fragment> {
        self.ast.clone()
    }

    /// Get the `Bodies` from the [`CommitMessage`]
    ///
    /// This gets the [`Bodies`] from the [`CommitMessage`] in easy to use
    /// paragraphs, we add in blank bodies because starting a new paragraph
    /// is a visual delimiter so we want to make that easy to detect.
    ///
    /// It doesn't include the [`Subject`] line, but if there's a blank line
    /// after it (as is recommended by the manual), the [`Bodies`] will
    /// start with a new empty [`Body`].
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Bodies, Body, CommitMessage, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Update bashrc to include kubernetes completions
    ///
    ///     This should make it easier to deploy things for the developers.
    ///     Benchmarked with Hyperfine, no noticable performance decrease.
    ///
    ///     I am unsure as to why this wasn't being automatically discovered from Brew.
    ///     I've filed a bug report with them.
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ));
    /// let bodies = vec![
    ///     Body::default(),
    ///     Body::from(indoc!(
    ///         "
    ///         This should make it easier to deploy things for the developers.
    ///         Benchmarked with Hyperfine, no noticable performance decrease."
    ///     )),
    ///     Body::default(),
    ///     Body::from(indoc!(
    ///         "
    ///         I am unsure as to why this wasn't being automatically discovered from Brew.
    ///         I've filed a bug report with them."
    ///     )),
    /// ];
    /// assert_eq!(message.get_body(), Bodies::from(bodies))
    /// ```
    #[must_use]
    pub fn get_body(&self) -> Bodies {
        self.bodies.clone()
    }

    /// Get the [`Comments`] from the [`CommitMessage`]
    ///
    /// We this will get you all the comments before the `Scissors` section. The
    /// [`Scissors`] section is the bit that appears when you run `git commit
    /// --verbose`, that contains the diffs.
    ///
    /// If there's [`Comment`] mixed in with the body, it'll return those too,
    /// but not any of the [`Body`] around them.
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Body, Comment, Comments, CommitMessage, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Update bashrc to include kubernetes completions
    ///
    ///     This should make it easier to deploy things for the developers.
    ///     Benchmarked with Hyperfine, no noticable performance decrease.
    ///
    ///     I am unsure as to why this wasn't being automatically discovered from Brew.
    ///     I've filed a bug report with them.
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ));
    /// let comments = vec![Comment::from(indoc!(
    ///     "
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ))];
    /// assert_eq!(message.get_comments(), Comments::from(comments))
    /// ```
    #[must_use]
    pub fn get_comments(&self) -> Comments {
        self.comments.clone()
    }

    /// Get the [`Scissors`] from the [`CommitMessage`]
    ///
    /// We this will get you all the comments in the [`Scissors`] section. The
    /// [`Scissors`] section is the bit that appears when you run `git commit
    /// --verbose`, that contains the diffs, and is not preserved when you
    /// save the commit.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Body, CommitMessage, Scissors, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Add file
    ///
    ///     This adds file primarily for demonstration purposes. It might not be
    ///     useful as an actual commit, but it's very useful as a example to use in
    ///     tests.
    ///
    ///     Relates-to: #128
    ///
    ///     ; Short (50 chars or less) summary of changes
    ///     ;
    ///     ; More detailed explanatory text, if necessary.  Wrap it to
    ///     ; about 72 characters or so.  In some contexts, the first
    ///     ; line is treated as the subject of an email and the rest of
    ///     ; the text as the body.  The blank line separating the
    ///     ; summary from the body is critical (unless you omit the body
    ///     ; entirely); tools like rebase can get confused if you run
    ///     ; the two together.
    ///     ;
    ///     ; Further paragraphs come after blank lines.
    ///     ;
    ///     ;   - Bullet points are okay, too
    ///     ;
    ///     ;   - Typically a hyphen or asterisk is used for the bullet,
    ///     ;     preceded by a single space, with blank lines in
    ///     ;     between, but conventions vary here
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Auf Branch main
    ///     ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;   neue Datei:     file
    ///     ;
    ///     ; ------------------------ >8 ------------------------
    ///     ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    ///     ; Alles unterhalb von ihr wird ignoriert.
    ///     diff --git a/file b/file
    ///     new file mode 100644
    ///     index 0000000..e69de29
    ///     "
    /// ));
    /// let scissors = Scissors::from(indoc!(
    ///     "
    ///     ; ------------------------ >8 ------------------------
    ///     ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    ///     ; Alles unterhalb von ihr wird ignoriert.
    ///     diff --git a/file b/file
    ///     new file mode 100644
    ///     index 0000000..e69de29
    ///     "
    /// ));
    /// assert_eq!(message.get_scissors(), Some(scissors))
    /// ```
    #[must_use]
    pub fn get_scissors(&self) -> Option<Scissors> {
        self.scissors.clone()
    }

    /// Get the [`Scissors`] from the [`CommitMessage`]
    ///
    /// We this will get you all the comments in the [`Scissors`] section. The
    /// [`Scissors`] section is the bit that appears when you run `git commit
    /// --verbose`, that contains the diffs, and is not preserved when you
    /// save the commit.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Body, CommitMessage, Trailer, Trailers};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Add file
    ///
    ///     Looks-like-a-trailer: But isn't
    ///
    ///     This adds file primarily for demonstration purposes. It might not be
    ///     useful as an actual commit, but it's very useful as a example to use in
    ///     tests.
    ///
    ///     Relates-to: #128
    ///     Relates-to: #129
    ///
    ///     ; Short (50 chars or less) summary of changes
    ///     ;
    ///     ; More detailed explanatory text, if necessary.  Wrap it to
    ///     ; about 72 characters or so.  In some contexts, the first
    ///     ; line is treated as the subject of an email and the rest of
    ///     ; the text as the body.  The blank line separating the
    ///     ; summary from the body is critical (unless you omit the body
    ///     ; entirely); tools like rebase can get confused if you run
    ///     ; the two together.
    ///     ;
    ///     ; Further paragraphs come after blank lines.
    ///     ;
    ///     ;   - Bullet points are okay, too
    ///     ;
    ///     ;   - Typically a hyphen or asterisk is used for the bullet,
    ///     ;     preceded by a single space, with blank lines in
    ///     ;     between, but conventions vary here
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Auf Branch main
    ///     ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;   neue Datei:     file
    ///     ;
    ///     ; ------------------------ >8 ------------------------
    ///     ; \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
    ///     ; Alles unterhalb von ihr wird ignoriert.
    ///     diff --git a/file b/file
    ///     new file mode 100644
    ///     index 0000000..e69de29
    ///     "
    /// ));
    /// let trailers = vec![
    ///     Trailer::new("Relates-to", "#128"),
    ///     Trailer::new("Relates-to", "#129"),
    /// ];
    /// assert_eq!(message.get_trailers(), Trailers::from(trailers))
    /// ```
    #[must_use]
    pub fn get_trailers(&self) -> Trailers {
        self.trailers.clone()
    }

    /// Does the [`CommitMessage`] the saved portions of the commit
    ///
    /// This takes a regex and matches it to the visible portions of the
    /// commits, so it excludes comments, and everything after the scissors.
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use regex::Regex;
    ///
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///
    ///     This is an example commit message for linting
    ///
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Auf Branch main
    ///     ; Ihr Branch ist auf demselben Stand wie 'origin/main'.
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     file
    ///     ;
    ///     "
    /// ));
    ///
    /// let re = Regex::new("[Bb]itte").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), false);
    ///
    /// let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), true);
    ///
    /// let re = Regex::new("[Ee]xample Commit Message").unwrap();
    /// assert_eq!(commit.matches_pattern(&re), true);
    /// ```
    #[must_use]
    pub fn matches_pattern(&self, re: &Regex) -> bool {
        let text = self
            .clone()
            .get_ast()
            .into_iter()
            .filter_map(|fragment| match fragment {
                Fragment::Body(body) => Some(String::from(body)),
                Fragment::Comment(_) => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        re.is_match(&text)
    }
}

impl From<CommitMessage> for String {
    fn from(commit_message: CommitMessage) -> Self {
        let basic_commit = commit_message
            .get_ast()
            .iter()
            .map(|item| match item {
                Fragment::Body(contents) => String::from(contents.clone()),
                Fragment::Comment(contents) => String::from(contents.clone()),
            })
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(scissors) = commit_message.get_scissors() {
            format!("{}\n{}", basic_commit, String::from(scissors))
        } else {
            basic_commit
        }
    }
}

impl From<&str> for CommitMessage {
    /// Create a new [`CommitMessage`]
    ///
    /// Create a commit message from a string. It's expected that you'll be
    /// reading this during some sort of Git Hook
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{Bodies, CommitMessage, Subject};
    ///
    /// let message = CommitMessage::from(indoc!(
    ///     "
    ///     Update bashrc to include kubernetes completions
    ///
    ///     This should make it easier to deploy things for the developers.
    ///     Benchmarked with Hyperfine, no noticable performance decrease.
    ///
    ///     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ///     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ///     ; bricht den Commit ab.
    ///     ;
    ///     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ///     ;
    ///     ; Auf Branch master
    ///     ;
    ///     ; Initialer Commit
    ///     ;
    ///     ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ///     ;    neue Datei:     .bashrc
    ///     ;"
    /// ));
    /// assert_eq!(
    ///     message.get_subject(),
    ///     Subject::from("Update bashrc to include kubernetes completions")
    /// )
    fn from(message: &str) -> Self {
        let (rest, scissors) = Scissors::parse_sections(message);
        let comment_character = CommitMessage::guess_comment_character(&rest, scissors.clone());
        let per_line_ast = CommitMessage::convert_to_per_line_ast(comment_character, &rest);
        let trailers = per_line_ast.clone().into();
        let mut ast: Vec<Fragment> = CommitMessage::group_ast(per_line_ast);

        if let (None, Some('\n')) = (scissors.clone(), message.chars().last()) {
            ast.push(Fragment::Body(Body::default()));
        }

        let subject = Subject::from(ast.clone());
        let comments = Comments::from(ast.clone());
        let bodies = Bodies::from(ast.clone());

        CommitMessage {
            scissors,
            ast,
            subject,
            trailers,
            comments,
            bodies,
        }
    }
}

impl TryFrom<PathBuf> for CommitMessage {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(Error::from)
            .map(move |_| CommitMessage::from(buffer))
    }
}

impl From<String> for CommitMessage {
    fn from(message: String) -> Self {
        let str: &str = &message;
        CommitMessage::from(str)
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to read commit file {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit::commit_message::error::io),
        help("check the file is readable")
    )]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use regex::Regex;

    use super::CommitMessage;
    use crate::{
        bodies::Bodies,
        body::Body,
        comment::Comment,
        comments::Comments,
        scissors::Scissors,
        subject::Subject,
        trailer::Trailer,
        trailers::Trailers,
        Fragment,
    };

    #[test]
    fn can_check_if_it_matches_pattern() {
        let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153
            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        let re = Regex::new("[Bb]itte").unwrap();
        assert!(!commit.matches_pattern(&re));

        let re = Regex::new("f[o\u{00FC}]r linting").unwrap();
        assert!(commit.matches_pattern(&re));

        let re = Regex::new("[Ee]xample Commit Message").unwrap();
        assert!(commit.matches_pattern(&re));

        let re = Regex::new("Relates[- ]to").unwrap();
        assert!(commit.matches_pattern(&re));
    }

    #[test]
    fn can_add_trailers_to_a_normal_commit() {
        let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        assert_eq!(
            String::from(commit.add_trailer(Trailer::new("Co-authored-by", "Test Trailer <test@example.com>"))),
            String::from(CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Relates-to: #153
            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ))));
    }

    #[test]
    fn can_add_trailers_to_a_commit_without_existing_trailers() {
        let commit = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        let expected = CommitMessage::from(indoc!(
            "
            Example Commit Message

            This is an example commit message for linting

            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
        assert_eq!(
            String::from(commit.add_trailer(Trailer::new(
                "Co-authored-by",
                "Test Trailer <test@example.com>",
            ))),
            String::from(expected)
        );
    }

    #[test]
    fn can_add_trailers_to_an_empty_commit() {
        let commit = CommitMessage::from(indoc!(
            "

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        let expected = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
        assert_eq!(
            String::from(commit.add_trailer(Trailer::new(
                "Co-authored-by",
                "Test Trailer <test@example.com>",
            ))),
            String::from(expected)
        );
    }

    #[test]
    fn can_add_trailers_to_an_empty_commit_with_single_trailer() {
        let commit = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));

        let expected = CommitMessage::from(indoc!(
            "


            Co-authored-by: Test Trailer <test@example.com>
            Co-authored-by: Someone Else <someone@example.com>

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch main
            # Ihr Branch ist auf demselben Stand wie 'origin/main'.
            #
            # Zum Commit vorgemerkte \u{00E4}nderungen:
            #	neue Datei:     file
            #
            "
        ));
        assert_eq!(
            String::from(commit.add_trailer(Trailer::new(
                "Co-authored-by",
                "Someone Else <someone@example.com>",
            ))),
            String::from(expected)
        );
    }

    #[test]
    fn can_generate_a_commit_from_an_ast() {
        let message = CommitMessage::from_fragments(
            vec![
                Fragment::Body(Body::from("Example Commit")),
                Fragment::Body(Body::default()),
                Fragment::Body(Body::from("Here is a body")),
                Fragment::Comment(Comment::from("# Example Commit")),
            ],
            Some(Scissors::from(indoc!(
                "
                # ------------------------ >8 ------------------------
                # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/file b/file
                new file mode 100644
                index 0000000..e69de29
                "
            ))),
        );

        assert_eq!(
            String::from(message),
            String::from(indoc!(
                "
                Example Commit

                Here is a body
                # Example Commit
                # ------------------------ >8 ------------------------
                # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/file b/file
                new file mode 100644
                index 0000000..e69de29
                "
            ))
        );
    }

    const COMMIT_WITH_ALL_FEATURES: &str = indoc!(
        "
        Add file

        Looks-like-a-trailer: But isn't

        This adds file primarily for demonstration purposes. It might not be
        useful as an actual commit, but it's very useful as a example to use in
        tests.

        Relates-to: #128

        # Short (50 chars or less) summary of changes
        #
        # More detailed explanatory text, if necessary.  Wrap it to
        # about 72 characters or so.  In some contexts, the first
        # line is treated as the subject of an email and the rest of
        # the text as the body.  The blank line separating the
        # summary from the body is critical (unless you omit the body
        # entirely); tools like rebase can get confused if you run
        # the two together.
        #
        # Further paragraphs come after blank lines.
        #
        #   - Bullet points are okay, too
        #
        #   - Typically a hyphen or asterisk is used for the bullet,
        #     preceded by a single space, with blank lines in
        #     between, but conventions vary here

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Auf Branch main
        # Ihr Branch ist auf demselben Stand wie 'origin/main'.
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     file
        #
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        new file mode 100644
        index 0000000..e69de29
        "
    );

    #[test]
    fn can_reliably_parse_from_commit_with_all_features() {
        let first_commit_message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, COMMIT_WITH_ALL_FEATURES);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Add file")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Relates-to: #128")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn insert_after_last_body() {
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Add file")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Relates-to: #128")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
        ];
        let commit = CommitMessage::from_fragments(ast, None);

        assert_eq!(commit.insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))]).get_ast(), vec![
            Fragment::Body(Body::from("Add file")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Looks-like-a-trailer: But isn\'t")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This adds file primarily for demonstration purposes. It might not be\nuseful as an actual commit, but it\'s very useful as a example to use in\ntests.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Relates-to: #128\nRelates-to: #656")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
        ]);
    }

    #[test]
    fn insert_after_last_body_with_no_body() {
        let ast: Vec<Fragment> = vec![
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
        ];
        let commit = CommitMessage::from_fragments(ast, None);

        assert_eq!(commit.insert_after_last_full_body(vec![Fragment::Body(Body::from("Relates-to: #656"))]).get_ast(), vec![
            Fragment::Body(Body::from("Relates-to: #656")),
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch main\n# Ihr Branch ist auf demselben Stand wie \'origin/main\'.\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     file\n#")),
        ]);
    }

    #[test]
    fn can_get_subject_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

        assert_eq!(message.get_subject(), Subject::from("Add file"));
    }

    #[test]
    fn can_get_body_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from("Looks-like-a-trailer: But isn't"),
                Body::default(),
                Body::from(indoc!(
                    "
                    This adds file primarily for demonstration purposes. It might not be
                    useful as an actual commit, but it's very useful as a example to use in
                    tests."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

        assert_eq!(
            message.get_scissors(),
            Some(Scissors::from(indoc!(
                "
                # ------------------------ >8 ------------------------
                # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/file b/file
                new file mode 100644
                index 0000000..e69de29
                "
            )))
        );
    }

    #[test]
    fn can_get_comments_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![
                Comment::from(indoc!(
                    "
                    # Short (50 chars or less) summary of changes
                    #
                    # More detailed explanatory text, if necessary.  Wrap it to
                    # about 72 characters or so.  In some contexts, the first
                    # line is treated as the subject of an email and the rest of
                    # the text as the body.  The blank line separating the
                    # summary from the body is critical (unless you omit the body
                    # entirely); tools like rebase can get confused if you run
                    # the two together.
                    #
                    # Further paragraphs come after blank lines.
                    #
                    #   - Bullet points are okay, too
                    #
                    #   - Typically a hyphen or asterisk is used for the bullet,
                    #     preceded by a single space, with blank lines in
                    #     between, but conventions vary here"
                )),
                Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Auf Branch main
                    # Ihr Branch ist auf demselben Stand wie 'origin/main'.
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     file
                    #"
                )),
            ])
        );
    }

    #[test]
    fn can_get_trailers_from_commit_with_all_features() {
        let message = CommitMessage::from(COMMIT_WITH_ALL_FEATURES);

        assert_eq!(
            message.get_trailers(),
            Trailers::from(vec![Trailer::new("Relates-to", "#128")])
        );
    }

    const LONG_SUBJECT_ONLY_COMMIT: &str = indoc!(
        "
        Initial Commit
        # Short (50 chars or less) summary of changes
        #
        # More detailed explanatory text, if necessary.  Wrap it to
        # about 72 characters or so.  In some contexts, the first
        # line is treated as the subject of an email and the rest of
        # the text as the body.  The blank line separating the
        # summary from the body is critical (unless you omit the body
        # entirely); tools like rebase can get confused if you run
        # the two together.
        #
        # Further paragraphs come after blank lines.
        #
        #   - Bullet points are okay, too
        #
        #   - Typically a hyphen or asterisk is used for the bullet,
        #     preceded by a single space, with blank lines in
        #     between, but conventions vary here

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     src/bodies.rs
        #	neue Datei:     src/body.rs
        #	neue Datei:     src/comment.rs
        #	neue Datei:     src/comments.rs
        #	neue Datei:     src/commit_message.rs
        #	neue Datei:     src/scissors.rs
        #	neue Datei:     src/subject.rs
        #	neue Datei:     src/trailer.rs
        #	neue Datei:     src/trailers.rs
        #
        # \u{00E4}nderungen, die nicht zum Commit vorgemerkt sind:
        #	ge\u{00E4}ndert:       src/bodies.rs
        #	ge\u{00E4}ndert:       src/body.rs
        #	ge\u{00E4}ndert:       src/comment.rs
        #	ge\u{00E4}ndert:       src/comments.rs
        #	ge\u{00E4}ndert:       src/commit_message.rs
        #	ge\u{00E4}ndert:       src/scissors.rs
        #	ge\u{00E4}ndert:       src/subject.rs
        #	ge\u{00E4}ndert:       src/trailer.rs
        #	ge\u{00E4}ndert:       src/trailers.rs
        #
        # Unversionierte Dateien:
        #	.gitignore
        #	Cargo.toml
        #	src/lib.rs
        #
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/src/bodies.rs b/src/bodies.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/body.rs b/src/body.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/comment.rs b/src/comment.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/comments.rs b/src/comments.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/commit_message.rs b/src/commit_message.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/scissors.rs b/src/scissors.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/subject.rs b/src/subject.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/trailer.rs b/src/trailer.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/trailers.rs b/src/trailers.rs
        new file mode 100644
        index 0000000..e69de29"
    );

    #[test]
    fn can_reliably_parse_from_subject_only_commit() {
        let first_commit_message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, LONG_SUBJECT_ONLY_COMMIT);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Initial Commit")),
            Fragment::Comment(Comment::from("# Short (50 chars or less) summary of changes\n#\n# More detailed explanatory text, if necessary.  Wrap it to\n# about 72 characters or so.  In some contexts, the first\n# line is treated as the subject of an email and the rest of\n# the text as the body.  The blank line separating the\n# summary from the body is critical (unless you omit the body\n# entirely); tools like rebase can get confused if you run\n# the two together.\n#\n# Further paragraphs come after blank lines.\n#\n#   - Bullet points are okay, too\n#\n#   - Typically a hyphen or asterisk is used for the bullet,\n#     preceded by a single space, with blank lines in\n#     between, but conventions vary here")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     src/bodies.rs\n#\tneue Datei:     src/body.rs\n#\tneue Datei:     src/comment.rs\n#\tneue Datei:     src/comments.rs\n#\tneue Datei:     src/commit_message.rs\n#\tneue Datei:     src/scissors.rs\n#\tneue Datei:     src/subject.rs\n#\tneue Datei:     src/trailer.rs\n#\tneue Datei:     src/trailers.rs\n#\n# \u{e4}nderungen, die nicht zum Commit vorgemerkt sind:\n#\tge\u{e4}ndert:       src/bodies.rs\n#\tge\u{e4}ndert:       src/body.rs\n#\tge\u{e4}ndert:       src/comment.rs\n#\tge\u{e4}ndert:       src/comments.rs\n#\tge\u{e4}ndert:       src/commit_message.rs\n#\tge\u{e4}ndert:       src/scissors.rs\n#\tge\u{e4}ndert:       src/subject.rs\n#\tge\u{e4}ndert:       src/trailer.rs\n#\tge\u{e4}ndert:       src/trailers.rs\n#\n# Unversionierte Dateien:\n#\t.gitignore\n#\tCargo.toml\n#\tsrc/lib.rs\n#")),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

        assert_eq!(message.get_subject(), Subject::from("Initial Commit"));
    }

    #[test]
    fn can_get_body_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
        let bodies: Vec<Body> = vec![];

        assert_eq!(message.get_body(), Bodies::from(bodies));
    }

    #[test]
    fn can_get_scissors_section_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

        assert_eq!(
            message.get_scissors(),
            Some(Scissors::from(indoc!(
                "
                # ------------------------ >8 ------------------------
                # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/src/bodies.rs b/src/bodies.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/body.rs b/src/body.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/comment.rs b/src/comment.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/comments.rs b/src/comments.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/commit_message.rs b/src/commit_message.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/scissors.rs b/src/scissors.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/subject.rs b/src/subject.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/trailer.rs b/src/trailer.rs
                new file mode 100644
                index 0000000..e69de29
                diff --git a/src/trailers.rs b/src/trailers.rs
                new file mode 100644
                index 0000000..e69de29"
            )))
        );
    }

    #[test]
    fn can_get_comments_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![
                Comment::from(indoc!(
                    "
                    # Short (50 chars or less) summary of changes
                    #
                    # More detailed explanatory text, if necessary.  Wrap it to
                    # about 72 characters or so.  In some contexts, the first
                    # line is treated as the subject of an email and the rest of
                    # the text as the body.  The blank line separating the
                    # summary from the body is critical (unless you omit the body
                    # entirely); tools like rebase can get confused if you run
                    # the two together.
                    #
                    # Further paragraphs come after blank lines.
                    #
                    #   - Bullet points are okay, too
                    #
                    #   - Typically a hyphen or asterisk is used for the bullet,
                    #     preceded by a single space, with blank lines in
                    #     between, but conventions vary here"
                )),
                Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     src/bodies.rs
                    #	neue Datei:     src/body.rs
                    #	neue Datei:     src/comment.rs
                    #	neue Datei:     src/comments.rs
                    #	neue Datei:     src/commit_message.rs
                    #	neue Datei:     src/scissors.rs
                    #	neue Datei:     src/subject.rs
                    #	neue Datei:     src/trailer.rs
                    #	neue Datei:     src/trailers.rs
                    #
                    # \u{00E4}nderungen, die nicht zum Commit vorgemerkt sind:
                    #	ge\u{00E4}ndert:       src/bodies.rs
                    #	ge\u{00E4}ndert:       src/body.rs
                    #	ge\u{00E4}ndert:       src/comment.rs
                    #	ge\u{00E4}ndert:       src/comments.rs
                    #	ge\u{00E4}ndert:       src/commit_message.rs
                    #	ge\u{00E4}ndert:       src/scissors.rs
                    #	ge\u{00E4}ndert:       src/subject.rs
                    #	ge\u{00E4}ndert:       src/trailer.rs
                    #	ge\u{00E4}ndert:       src/trailers.rs
                    #
                    # Unversionierte Dateien:
                    #	.gitignore
                    #	Cargo.toml
                    #	src/lib.rs
                    #"
                )),
            ])
        );
    }

    #[test]
    fn can_get_trailers_from_subject_only_commit() {
        let message = CommitMessage::from(LONG_SUBJECT_ONLY_COMMIT);
        let trailers: Vec<Trailer> = Vec::default();

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const NOT_VERBOSE_COMMIT: &str = indoc!(
        "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Datum:            Sat Jun 27 21:40:14 2020 +0200
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     .bashrc
        #"
    );

    #[test]
    fn can_reliably_parse_from_not_verbose_commit() {
        let first_commit_message = CommitMessage::from(NOT_VERBOSE_COMMIT);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, NOT_VERBOSE_COMMIT);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

        assert_eq!(
            message.get_subject(),
            Subject::from("Update bashrc to include kubernetes completions")
        );
    }

    #[test]
    fn can_get_body_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This should make it easier to deploy things for the developers.
                    Benchmarked with Hyperfine, no noticable performance decrease."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![
                Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
            ])
        );
    }

    #[test]
    fn can_get_trailers_from_not_verbose_commit() {
        let message = CommitMessage::from(NOT_VERBOSE_COMMIT);
        let trailers: Vec<Trailer> = Vec::default();

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const NON_STANDARD_COMMENT_CHARACTER: &str = indoc!(
        "
        Allow the server to respond to https

        This allows the server to respond to HTTPS requests, by correcting the port binding.
        We should see a nice speed increase from this

        fixes:
        #6436
        #6437
        #6438

        ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
        ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
        ; bricht den Commit ab."
    );

    #[test]
    fn can_reliably_parse_from_non_standard_comment_char_commit() {
        let first_commit_message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, NON_STANDARD_COMMENT_CHARACTER);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Allow the server to respond to https")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This allows the server to respond to HTTPS requests, by correcting the port binding.\nWe should see a nice speed increase from this")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("fixes:\n#6436\n#6437\n#6438")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("; Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{c4}nderungen ein. Zeilen,\n; die mit \';\' beginnen, werden ignoriert, und eine leere Beschreibung\n; bricht den Commit ab.")),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

        assert_eq!(
            message.get_subject(),
            Subject::from("Allow the server to respond to https")
        );
    }

    #[test]
    fn can_get_body_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This allows the server to respond to HTTPS requests, by correcting the port binding.
                    We should see a nice speed increase from this"
                    )),
                Body::default(),
                Body::from(indoc!(
                    "
                    fixes:
                    #6436
                    #6437
                    #6438"
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![Comment::from(indoc!(
                "
                ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
                ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
                ; bricht den Commit ab."
            ))])
        );
    }

    #[test]
    fn can_get_trailers_from_non_standard_comment_char_commit() {
        let message = CommitMessage::from(NON_STANDARD_COMMENT_CHARACTER);
        let trailers: Vec<Trailer> = Vec::default();

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const MULTIPLE_TRAILERS: &str = indoc!(
        "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
        Co-authored-by: Somebody Else <somebody@example.com>

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Datum:            Sat Jun 27 21:40:14 2020 +0200
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     .bashrc
        #"
    );

    #[test]
    fn can_reliably_parse_from_multiple_trailers() {
        let first_commit_message = CommitMessage::from(MULTIPLE_TRAILERS);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, MULTIPLE_TRAILERS);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);

        assert_eq!(
            message.get_subject(),
            Subject::from("Update bashrc to include kubernetes completions")
        );
    }

    #[test]
    fn can_get_body_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This should make it easier to deploy things for the developers.
                    Benchmarked with Hyperfine, no noticable performance decrease."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![
                Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
            ])
        );
    }

    #[test]
    fn can_get_trailers_from_multiple_trailers() {
        let message = CommitMessage::from(MULTIPLE_TRAILERS);
        let trailers: Vec<Trailer> = vec![
            Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
            Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
        ];

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const TRAILING_EMPTY_NEWLINES: &str = indoc!(
        "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
        Co-authored-by: Somebody Else <somebody@example.com>

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Datum:            Sat Jun 27 21:40:14 2020 +0200
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     .bashrc
        #


        "
    );

    #[test]
    fn can_reliably_parse_from_trailing_empty_newlines() {
        let first_commit_message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, TRAILING_EMPTY_NEWLINES);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
            Fragment::Body(Body::default()),
            Fragment::Comment(Comment::from("# Bitte geben Sie eine Commit-Beschreibung f\u{fc}r Ihre \u{e4}nderungen ein. Zeilen,\n# die mit \'#\' beginnen, werden ignoriert, und eine leere Beschreibung\n# bricht den Commit ab.\n#\n# Datum:            Sat Jun 27 21:40:14 2020 +0200\n#\n# Auf Branch master\n#\n# Initialer Commit\n#\n# Zum Commit vorgemerkte \u{e4}nderungen:\n#\tneue Datei:     .bashrc\n#")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::default()),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

        assert_eq!(
            message.get_subject(),
            Subject::from("Update bashrc to include kubernetes completions")
        );
    }

    #[test]
    fn can_get_body_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This should make it easier to deploy things for the developers.
                    Benchmarked with Hyperfine, no noticable performance decrease."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);

        assert_eq!(
            message.get_comments(),
            Comments::from(vec![
                Comment::from(indoc!(
                    "
                    # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
                    # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
                    # bricht den Commit ab.
                    #
                    # Datum:            Sat Jun 27 21:40:14 2020 +0200
                    #
                    # Auf Branch master
                    #
                    # Initialer Commit
                    #
                    # Zum Commit vorgemerkte \u{00E4}nderungen:
                    #	neue Datei:     .bashrc
                    #"
                ))
            ])
        );
    }

    #[test]
    fn can_get_trailers_from_trailing_empty_newlines() {
        let message = CommitMessage::from(TRAILING_EMPTY_NEWLINES);
        let trailers: Vec<Trailer> = vec![
            Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
            Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
        ];

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const COMMIT_MESSAGE_WITH_NO_COMMENTS: &str = indoc!(
        "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
        Co-authored-by: Somebody Else <somebody@example.com>
        "
    );

    #[test]
    fn can_reliably_parse_from_a_commit_message_without_commits() {
        let first_commit_message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(string_version_of_commit, COMMIT_MESSAGE_WITH_NO_COMMENTS);
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\nCo-authored-by: Somebody Else <somebody@example.com>")),
            Fragment::Body(Body::default()),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

        assert_eq!(
            message.get_subject(),
            Subject::from("Update bashrc to include kubernetes completions")
        );
    }

    #[test]
    fn can_get_body_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This should make it easier to deploy things for the developers.
                    Benchmarked with Hyperfine, no noticable performance decrease."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
        let comments: Vec<Comment> = vec![];

        assert_eq!(message.get_comments(), Comments::from(comments));
    }

    #[test]
    fn can_get_trailers_from_a_commit_message_without_commits() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_NO_COMMENTS);
        let trailers: Vec<Trailer> = vec![
            Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
            Trailer::new("Co-authored-by", "Somebody Else <somebody@example.com>"),
        ];

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }

    const COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE: &str = indoc!(
        "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
         Co-authored-by: Somebody Else <somebody@example.com>
        "
    );

    #[test]
    fn can_reliably_parse_from_a_commit_message_with_trailing_whitespace() {
        let first_commit_message =
            CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
        let string_version_of_commit = String::from(first_commit_message.clone());
        let second_commit_message = CommitMessage::from(string_version_of_commit.clone());

        assert_eq!(
            string_version_of_commit,
            COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE
        );
        assert_eq!(first_commit_message, second_commit_message);
    }

    #[test]
    fn can_get_ast_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
        let ast: Vec<Fragment> = vec![
            Fragment::Body(Body::from("Update bashrc to include kubernetes completions")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("This should make it easier to deploy things for the developers.\nBenchmarked with Hyperfine, no noticable performance decrease.")),
            Fragment::Body(Body::default()),
            Fragment::Body(Body::from("Co-authored-by: Billie Thomposon <billie@example.com>\n Co-authored-by: Somebody Else <somebody@example.com>")),
            Fragment::Body(Body::default()),
        ];

        assert_eq!(message.get_ast(), ast);
    }

    #[test]
    fn can_get_subject_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

        assert_eq!(
            message.get_subject(),
            Subject::from("Update bashrc to include kubernetes completions")
        );
    }

    #[test]
    fn can_get_body_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

        assert_eq!(
            message.get_body(),
            Bodies::from(vec![
                Body::default(),
                Body::from(indoc!(
                    "
                    This should make it easier to deploy things for the developers.
                    Benchmarked with Hyperfine, no noticable performance decrease."
                )),
            ])
        );
    }

    #[test]
    fn can_get_scissors_section_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);

        assert_eq!(message.get_scissors(), None);
    }

    #[test]
    fn can_get_comments_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
        let comments: Vec<Comment> = vec![];

        assert_eq!(message.get_comments(), Comments::from(comments));
    }

    #[test]
    fn can_get_trailers_from_a_commit_message_with_trailing_whitespace() {
        let message = CommitMessage::from(COMMIT_MESSAGE_WITH_WHITESPACE_STARTING_LAST_LINE);
        let trailers: Vec<Trailer> = vec![
            Trailer::new("Co-authored-by", "Billie Thomposon <billie@example.com>"),
            Trailer::new(" Co-authored-by", "Somebody Else <somebody@example.com>"),
        ];

        assert_eq!(message.get_trailers(), Trailers::from(trailers));
    }
}

lazy_static! {
    static ref NOT_WHITESPACE_RE: Regex = Regex::new("^\\W$").unwrap();
}

impl CommitMessage {
    fn guess_comment_character(rest: &str, scissors: Option<Scissors>) -> Option<char> {
        match scissors {
            Some(scissors) => String::from(scissors).chars().next(),
            None => rest
                .lines()
                .rev()
                .find(|line| !line.trim().is_empty())
                .and_then(|line| line.chars().next())
                .filter(|x| NOT_WHITESPACE_RE.is_match(x.to_string().trim())),
        }
    }
}
