#[cfg(test)]
mod number_of_operations_to_make_network_connected {
    use solver::number_of_operations_to_make_network_connected::make_connected;
    #[test]
    fn example1() {
        assert_eq!(
            make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]),
            0
        );
    }
    // #[test]
    // fn example1() {
    //     assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    // }

    // #[test]
    // fn example2() {
    //     assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    // }

    // #[test]
    // fn example3() {
    //     assert_eq!(jump(vec![1, 2, 1, 1, 1]), 3);
    // }
}
