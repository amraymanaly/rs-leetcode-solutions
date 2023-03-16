use solver::group_anagrams::group_anagrams;
fn main() {
    let x = group_anagrams(vec![
        "eat".into(),
        "tea".into(),
        "tan".into(),
        "ate".into(),
        "nat".into(),
        "bat".into(),
    ]);

    println!("{:?}", x);
}
