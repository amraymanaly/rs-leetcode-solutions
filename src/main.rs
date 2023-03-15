use solver::regular_expression_matching::is_match;
fn main() {
    // let p = "012345678";
    // println!("{}", &p[6..11]);
    let res = is_match("aaaaaaaaaaaaaaaaaaab".into(), "a*a*a*a*a*a*a*a*a*a*".into());
    println!("Got {} in {} rounds!", res.0, res.1);
}
