#[cfg(test)]
mod tests {

    #[test]
    fn test_cast() {
        //You can only use the `+` operator to add values of the same type together.
        let a = 5 + 5;
        assert_eq!(a, 10);

        // Integers and floating-point numbers have different types,
        // so using the + operator to add them will result in a build error.
        //let b = 5 + 5.0;
        //println!("b: {}", b);

        // When adding numbers of different types,
        // you need to use `as` to explicitly convert the type.
        let b = 5 as f32 + 5.0;
        assert_eq!(b, 10.0);
    }
}
