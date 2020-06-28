//! A library to parse commit messages in git hooks
//!
//! Make it a bit easier to write lints and for git hooks
//!
//! # Example
//!
//! ```
//! use indoc::indoc;
//! use mit_commit::{Bodies, CommitMessage, Subject};
//!
//! let message = CommitMessage::from(indoc!(
//!     "
//!     Update bashrc to include kubernetes completions
//!
//!     This should make it easier to deploy things for the developers.
//!     Benchmarked with Hyperfine, no noticable performance decrease.
//!
//!     ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
//!     ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
//!     ; bricht den Commit ab.
//!     ;
//!     ; Datum:            Sat Jun 27 21:40:14 2020 +0200
//!     ;
//!     ; Auf Branch master
//!     ;
//!     ; Initialer Commit
//!     ;
//!     ; Zum Commit vorgemerkte \u{00E4}nderungen:
//!     ;    neue Datei:     .bashrc
//!     ;"
//! ));
//! assert_eq!(
//!     message.get_subject(),
//!     Subject::from("Update bashrc to include kubernetes completions")
//! )
//! ```

mod bodies;
mod body;
mod comment;
mod comments;
mod commit_message;
mod fragment;
mod scissors;
mod subject;
mod trailer;
mod trailers;

/// A collection of user input `CommitMessage` text
pub use bodies::Bodies;

/// A single contiguous block of `CommitMessage` text
pub use body::Body;

/// A single comment from a `CommitMessage`
pub use comment::Comment;

/// A collection of comments from a `CommitMessage`
pub use comments::Comments;

/// A `CommitMessage`, the primary entry point to the library
pub use commit_message::CommitMessage;

/// A `Fragment` from the `CommitMessage`, either a comment or body
pub use fragment::Fragment;

/// The `Scissors` from a `CommitMessage`
pub use scissors::Scissors;

/// The `Subject` from the `CommitMessage`
pub use subject::Subject;

/// A `Trailer` you might see a in a `CommitMessage`, for example 'Co-authored-by: Billie Thompson <billie@example.com>'
pub use trailer::Trailer;

/// A Collection of `Trailer`
pub use trailers::Trailers;
