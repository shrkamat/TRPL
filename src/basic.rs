#[cfg(test)]
mod math {
    #[test]
    fn operators() {
        assert_eq!(2 + 2, 4);
        assert_eq!(2 - 2, 0);
        assert_eq!(2 * 2, 4);
        assert_eq!(2 % 2, 0);
    }
}
