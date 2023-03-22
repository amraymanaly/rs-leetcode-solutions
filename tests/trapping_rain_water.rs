#[cfg(test)]
mod trapping_rain_water {
    use solver::trapping_rain_water::trap;

    #[test]
    fn example1() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(trap(vec![2, 0, 2]), 2);
    }
}
