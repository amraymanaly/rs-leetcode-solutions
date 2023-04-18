#[cfg(test)]
mod simplify_path {
    use solver::simplify_path::simplify_path;

    #[test]
    fn example1() {
        assert_eq!(simplify_path("/../".into()), "/".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(simplify_path("/home/amr/".into()), "/home/amr".to_string());
    }
}
