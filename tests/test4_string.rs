#[cfg(test)]
mod tests {

    fn print_str(s: &str) {
        println!("{}", s);
    }

    #[test]
    fn test_string() {
        // The type of a string literal is &str.
        let str_ref = "hello";
        assert_eq!(str_ref, "hello");

        // You cannot modify a value of type &str.
        // To change the value of a string, you must use the String type.
        let mut string = String::new();
        string.push_str("Hello, ");
        string.push_str("World!");
        assert_eq!(string, "Hello, World!");

        // print_str requires a &str as the argument type,
        // but it is possible to pass a value of &String type.
        print_str(str_ref);
        print_str(&string);

        // The method len is defined for &str,
        // but it can also be called from a value of type String.
        assert_eq!(str_ref.len(), 5);
        assert_eq!(string.len(), 13);
    }
}
