#[cfg(test)]
mod three_sum {
    use solver::three_sum::three_sum;

    #[test]
    fn example1() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 0, 1], vec![2, -1, -1]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn example5() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn example4() {
        assert_eq!(
            three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![vec![]]
        )
    }
}
