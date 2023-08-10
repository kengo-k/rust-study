#[cfg(test)]
mod tests {

    #[test]
    fn test_func() {
        assert_eq!(power(2, 3), 8);
        assert_eq!(even_or_odd(8), "even");
        assert_eq!(even_or_odd(11), "odd");
        assert_eq!(greet1("World"), ());
        assert_eq!(greet2("Rust"), ());
    }

    // calculate the power
    //
    // The types of the arguments and return value of a function must be explicitly stated, and cannot be omitted.
    fn power(base: i32, exponent: u32) -> i32 {
        let mut result = 1;
        for _ in 0..exponent {
            result *= base;
        }

        // An expression placed at the end of a function becomes the return value.
        // Do not put a semicolon at the end of the expression.
        result
    }

    // Determining whether a number is even or odd
    fn even_or_odd(number: i32) -> String {
        // When terminating a function midway and returning a value, use the return keyword.
        // Since return is a statement, it must be followed by a semicolon.
        if number % 2 == 0 {
            return "even".to_string();
        }
        "odd".to_string()
    }

    // If the return type of a function is not described,
    // it is treated as having a return type of ().
    fn greet1(message: &str) {
        println!("Hello, {}!", message);
    }

    // It is also possible to explicitly specify () as the return type.
    fn greet2(message: &str) -> () {
        println!("Hello, {}!!", message);
    }
}
