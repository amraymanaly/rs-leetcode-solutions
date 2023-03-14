use solver::regular_expression_matching::is_match;
fn main() {
    let res = is_match("aaaaaaaaaaaaaaaaaaab".into(), "a*a*a*a*a*a*a*a*a*a*".into());
    println!("Got {res}!");
}
