#[cfg(test)]
mod container_with_most_water {
    use solver::container_with_most_water::max_area;

    #[test]
    fn example1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
