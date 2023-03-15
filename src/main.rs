use solver::longest_common_prefix::longest_common_prefix;
fn main() {
    println!(
        "answer is \"{}\"",
        longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()])
    );
}
