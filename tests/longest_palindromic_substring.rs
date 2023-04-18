#[cfg(test)]
mod longest_palindromic_substring {
    use solver::longest_palindromic_substring::longest_palindrome;

    #[test]
    fn example1() {
        assert_eq!(longest_palindrome("babad".into()), "bab".into());
    }

    #[test]
    fn example2() {
        assert_eq!(longest_palindrome("cbbd".into()), "bb".into());
    }
}
