use std::{
    borrow::Cow,
    convert::TryFrom,
    fs::File,
    io,
    io::Read,
    path::{Path, PathBuf},
};

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
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CommitMessage<'a> {
    scissors: Option<Scissors<'a>>,
    ast: Vec<Fragment<'a>>,
    subject: Subject<'a>,
    trailers: Trailers<'a>,
    comments: Comments<'a>,
    bodies: Bodies<'a>,
}

impl<'a> CommitMessage<'a> {
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
    pub fn from_fragments(fragments: Vec<Fragment<'_>>, scissors: Option<Scissors<'_>>) -> Self {
        let body = fragments
            .into_iter()
            .map(|x| match x {
                Fragment::Body(contents) => String::from(contents),
                Fragment::Comment(contents) => String::from(contents),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let scissors: String = scissors
            .map(|contents| format!("\n{}", String::from(contents)))
            .unwrap_or_default();

        Self::from(format!("{body}{scissors}"))
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
    ///         "Co-authored-by".into(),
    ///         "Test Trailer <test@example.com>".into()
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
    pub fn add_trailer(&self, trailer: Trailer<'_>) -> Self {
        let mut fragments = Vec::new();

        if self.bodies.iter().all(Body::is_empty) && self.trailers.is_empty() {
            fragments.push(Body::default().into());
        }

        if self.trailers.is_empty() {
            fragments.push(Body::default().into());
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
    pub fn insert_after_last_full_body(&self, fragment: Vec<Fragment<'_>>) -> Self {
        let position = self.ast.iter().rposition(|fragment| match fragment {
            Fragment::Body(body) => !body.is_empty(),
            Fragment::Comment(_) => false,
        });

        let (before, after): (Vec<_>, Vec<_>) = position.map_or_else(
            || (vec![], self.ast.clone().into_iter().enumerate().collect()),
            |position| {
                self.ast
                    .clone()
                    .into_iter()
                    .enumerate()
                    .partition(|(index, _)| index <= &position)
            },
        );

        Self::from_fragments(
            [
                before.into_iter().map(|(_, x)| x).collect(),
                fragment,
                after.into_iter().map(|(_, x)| x).collect(),
            ]
            .concat(),
            self.get_scissors(),
        )
    }

    fn convert_to_per_line_ast(comment_character: Option<char>, rest: &str) -> Vec<Fragment<'a>> {
        rest.lines()
            .map(|line| {
                comment_character.map_or_else(
                    || Body::from(line.to_string()).into(),
                    |comment_character| {
                        if line.starts_with(comment_character) {
                            Comment::from(line.to_string()).into()
                        } else {
                            Body::from(line.to_string()).into()
                        }
                    },
                )
            })
            .collect()
    }

    fn group_ast(ungrouped_ast: Vec<Fragment<'a>>) -> Vec<Fragment<'a>> {
        ungrouped_ast
            .into_iter()
            .fold(vec![], |acc: Vec<Fragment<'_>>, new_fragment| {
                let mut previous_fragments = acc.clone();
                match (acc.last(), &new_fragment) {
                    (None, fragment) => {
                        previous_fragments.push(fragment.clone());
                        previous_fragments
                    }
                    (Some(Fragment::Comment(existing)), Fragment::Comment(new)) => {
                        previous_fragments.truncate(acc.len() - 1);
                        previous_fragments.push(existing.append(new).into());
                        previous_fragments
                    }
                    (Some(Fragment::Body(existing)), Fragment::Body(new)) => {
                        if new.is_empty() || existing.is_empty() {
                            previous_fragments.push(Fragment::from(new.clone()));
                        } else {
                            previous_fragments.truncate(acc.len() - 1);
                            previous_fragments.push(existing.append(new).into());
                        }
                        previous_fragments
                    }
                    (Some(Fragment::Body(_)), Fragment::Comment(new)) => {
                        previous_fragments.push(Fragment::from(new.clone()));
                        previous_fragments
                    }
                    (Some(Fragment::Comment(_)), Fragment::Body(new)) => {
                        previous_fragments.push(Fragment::from(new.clone()));
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
    pub fn get_subject(&self) -> Subject<'a> {
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
    pub fn get_ast(&self) -> Vec<Fragment<'_>> {
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
    pub fn get_body(&self) -> Bodies<'_> {
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
    pub fn get_comments(&self) -> Comments<'_> {
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
    pub fn get_scissors(&self) -> Option<Scissors<'_>> {
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
    ///     Trailer::new("Relates-to".into(), "#128".into()),
    ///     Trailer::new("Relates-to".into(), "#129".into()),
    /// ];
    /// assert_eq!(message.get_trailers(), Trailers::from(trailers))
    /// ```
    #[must_use]
    pub fn get_trailers(&self) -> Trailers<'_> {
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

    fn guess_comment_character(message: &str) -> Option<char> {
        Scissors::guess_comment_character(message)
    }

    /// Give you a new [`CommitMessage`] with the provided subject
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{CommitMessage, Subject};
    /// use regex::Regex;
    ///
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///
    ///     This is an example commit message
    ///     "
    /// ));
    ///
    /// assert_eq!(
    ///     commit.with_subject("Subject".into()).get_subject(),
    ///     Subject::from("Subject")
    /// );
    /// ```
    #[must_use]
    pub fn with_subject(self, subject: Subject<'a>) -> Self {
        let mut ast: Vec<Fragment<'a>> = self.ast.clone();

        if !ast.is_empty() {
            ast.remove(0);
        }
        ast.insert(0, Body::from(subject.to_string()).into());

        Self {
            scissors: self.scissors,
            ast,
            subject,
            trailers: self.trailers,
            comments: self.comments,
            bodies: self.bodies,
        }
    }

    /// Give you a new [`CommitMessage`] with the provided body
    ///
    /// # Examples
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{CommitMessage, Subject};
    /// use regex::Regex;
    ///
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///
    ///     This is an example commit message
    ///     "
    /// ));
    /// let expected = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///
    ///     New body"
    /// ));
    ///
    /// assert_eq!(commit.with_body_contents("New body"), expected);
    /// ```
    ///
    /// A note on what we consider the body. The body is what falls after the
    /// gutter. This means the following behaviour might happen
    ///
    /// ```
    /// use indoc::indoc;
    /// use mit_commit::{CommitMessage, Subject};
    /// use regex::Regex;
    /// let commit = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///     without gutter"
    /// ));
    /// let expected = CommitMessage::from(indoc!(
    ///     "
    ///     Example Commit Message
    ///     without gutter
    ///
    ///     New body"
    /// ));
    ///
    /// assert_eq!(commit.with_body_contents("New body"), expected);
    /// ```
    #[must_use]
    pub fn with_body_contents(self, contents: &'a str) -> Self {
        let existing_subject: Subject<'a> = self.get_subject();
        let body = format!("Unused\n\n{contents}");
        let commit = Self::from(body);

        commit.with_subject(existing_subject)
    }

    /// Give you a new [`CommitMessage`] with the provided body
    ///
    /// # Examples
    ///
    /// ```
    /// use mit_commit::{CommitMessage, Subject};
    /// let commit = CommitMessage::from("No comment\n\n# Some Comment");
    ///
    /// assert_eq!(commit.get_comment_char().unwrap(), '#');
    /// ```
    ///
    /// We return none is there is no comments
    ///
    /// ```
    /// use mit_commit::{CommitMessage, Subject};
    /// let commit = CommitMessage::from("No comment");
    ///
    /// assert!(commit.get_comment_char().is_none());
    /// ```
    #[must_use]
    pub fn get_comment_char(&self) -> Option<char> {
        self.comments
            .iter()
            .next()
            .map(|comment| -> String { comment.clone().into() })
            .and_then(|comment| comment.chars().next())
    }
}

impl<'a> From<CommitMessage<'a>> for String {
    fn from(commit_message: CommitMessage<'_>) -> Self {
        let basic_commit = commit_message
            .get_ast()
            .iter()
            .map(|item| match item {
                Fragment::Body(contents) => Self::from(contents.clone()),
                Fragment::Comment(contents) => Self::from(contents.clone()),
            })
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(scissors) = commit_message.get_scissors() {
            format!("{basic_commit}\n{}", Self::from(scissors))
        } else {
            basic_commit
        }
    }
}

impl<'a> From<Cow<'a, str>> for CommitMessage<'a> {
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
    /// ```
    ///
    ///  # Comment Character
    ///
    /// We load the comment character for the commit message
    ///
    /// Valid options are in [`LEGAL_CHARACTERS`], these are the 'auto" selection logic in the git codebase in the [`adjust_comment_line_char`](https://github.com/git/git/blob/master/builtin/commit.c#L667-L695) function.
    ///
    /// This does mean that we aren't making 100% of characters available, which
    /// is technically possible, but given we don't have access to the users git
    /// config this feels like a reasonable compromise, there are a lot of
    /// non-whitespace characters as options otherwise, and we don't want to
    /// confuse a genuine body with a comment
    fn from(message: Cow<'a, str>) -> Self {
        let (rest, scissors) = Scissors::parse_sections(&message);
        let comment_character = Self::guess_comment_character(&message);
        let per_line_ast = Self::convert_to_per_line_ast(comment_character, &rest);
        let trailers = per_line_ast.clone().into();
        let mut ast: Vec<Fragment<'_>> = Self::group_ast(per_line_ast);

        if (scissors.clone(), message.chars().last()) == (None, Some('\n')) {
            ast.push(Body::default().into());
        }

        let subject = Subject::from(ast.clone());
        let comments = Comments::from(ast.clone());
        let bodies = Bodies::from(ast.clone());

        Self {
            scissors,
            ast,
            subject,
            trailers,
            comments,
            bodies,
        }
    }
}

impl<'a> TryFrom<PathBuf> for CommitMessage<'a> {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(Error::from)
            .map(move |_| Self::from(buffer))
    }
}

impl<'a> TryFrom<&'a Path> for CommitMessage<'a> {
    type Error = Error;

    fn try_from(value: &'a Path) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(Error::from)
            .map(move |_| Self::from(buffer))
    }
}

impl<'a> From<&'a str> for CommitMessage<'a> {
    fn from(message: &'a str) -> Self {
        CommitMessage::from(Cow::from(message))
    }
}

impl<'a> From<String> for CommitMessage<'a> {
    fn from(message: String) -> Self {
        Self::from(Cow::from(message))
    }
}

/// Errors on reading c commits
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// Failed to read a commit message
    #[error("failed to read commit file {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit::commit_message::error::io),
        help("check the file is readable")
    )]
    Io(#[from] io::Error),
}
