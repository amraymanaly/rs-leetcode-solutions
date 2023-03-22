#[cfg(test)]
mod jump_game_2 {
    use solver::jump_game_2::jump;

    #[test]
    fn example1() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(jump(vec![1, 2, 1, 1, 1]), 3);
    }
}
