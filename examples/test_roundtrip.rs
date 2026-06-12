use mit_commit::CommitMessage;

fn main() {
    let cases = vec![
        "Subject\n\nBody\n",
        "Subject\n\nBody\n\n",
        "Subject\n\nBody\n\n\n",
        "Subject\n",
        "Subject\n\n",
    ];
    for case in &cases {
        let msg = CommitMessage::from(*case);
        let s = String::from(msg);
        println!("Input:  {:?}", case);
        println!("Output: {:?}", s);
        println!("Match:  {}\n", s == *case);
    }
}
