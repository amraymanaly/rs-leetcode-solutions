#[cfg(test)]
mod regular_expression_matching {
    use solver::regular_expression_matching::is_match;

    #[test]
    fn example1() {
        assert_eq!(is_match("aa".into(), "aa".into()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_match("aa".into(), "a*".into()), true);
    }

    #[test]
    fn example3() {
        assert_eq!(is_match("b".into(), "ab*".into()), false);
    }

    #[test]
    fn example4() {
        assert_eq!(is_match("b".into(), "b*b*ba*".into()), true);
    }

    #[test]
    fn example5() {
        assert_eq!(is_match("b".into(), "b*b+b*".into()), true);
    }
}
