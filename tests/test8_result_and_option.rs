#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    #[test]
    fn test_result1() {
        // parse can fail, so it returns a `Result`."
        let input1 = "100";
        let n1: Result<i32, ParseIntError> = input1.parse();

        // If the parse is successful, `Ok` is returned.
        assert_eq!(n1, Ok(100));
        assert_eq!(n1.is_ok(), true);

        let input2 = "X";
        let n2: Result<i32, ParseIntError> = input2.parse();
        assert_eq!(n2.is_err(), true);
    }
}
