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

#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::pedantic,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub use bodies::Bodies;
pub use body::Body;
pub use comment::Comment;
pub use comments::Comments;
pub use commit_message::{CommitMessage, Error as CommitMessageError};
pub use fragment::Fragment;
pub use scissors::Scissors;
pub use subject::Subject;
pub use trailer::{Error as TrailerError, Trailer};
pub use trailers::Trailers;

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

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            unsafe extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
