# mit-commit

A library to parse commit messages in git hooks

``` rust
use indoc::indoc;
use mit_commit::{Bodies, CommitMessage, Subject};

let message = CommitMessage::from(indoc!(
    "
    Update bashrc to include kubernetes completions

    This should make it easier to deploy things for the developers.
    Benchmarked with Hyperfine, no noticable performance decrease.

    ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
    ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
    ; bricht den Commit ab.
    ;
    ; Datum:            Sat Jun 27 21:40:14 2020 +0200
    ;
    ; Auf Branch master
    ;
    ; Initialer Commit
    ;
    ; Zum Commit vorgemerkte \u{00E4}nderungen:
    ;    neue Datei:     .bashrc
    ;"
));
assert_eq!(
    message.get_subject(),
    Subject::from("Update bashrc to include kubernetes completions")
)
```

Read more at [Docs.rs](https://docs.rs/mit-commit/) or visit the [Codeberg repository](https://codeberg.org/PurpleBooth/mit-commit)
