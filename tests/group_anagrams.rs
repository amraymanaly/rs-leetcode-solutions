#[cfg(test)]
mod group_anagrams {
    use solver::group_anagrams::group_anagrams;

    #[test]
    fn example1() {
        assert_eq!(
            group_anagrams(vec![
                "eat".into(),
                "tea".into(),
                "tan".into(),
                "ate".into(),
                "nat".into(),
                "bat".into()
            ]),
            vec![vec![String::from("sdf")]]
        );
    }
}
