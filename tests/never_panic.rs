use mit_commit::CommitMessage;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[allow(unused_must_use)]
#[quickcheck]
fn never_panic(input: String) -> bool {
    CommitMessage::from(input);
    true
}
