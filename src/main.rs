use solver::regular_expression_matching::is_match;
fn main() {
    let res = is_match("b".into(), "a*".into());
    println!("Got {res}!");
}
