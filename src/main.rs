use solver::longest_common_prefix::longest_common_prefix;
// use solver::regular_expression_matching::is_match;
fn main() {
    println!(
        "answer is \"{}\"",
        longest_common_prefix(vec!["flowers".into(), "flow".into(), "flick".into()])
    );
    // let x = is_match("aaaaaaaaaaaaaaaaaaab".into(), "a*.*.+a+".into());
    // println!("Got {} in {} rounds!", x.0, x.1);
}
