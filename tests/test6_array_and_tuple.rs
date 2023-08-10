#[cfg(test)]
mod tests {

    #[test]
    fn test_array_and_tuple() {
        // The arrays in Rust are of fixed length,
        // and their type is denoted as [element type; number of elements].
        let arr1: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr1, [1, 2, 3, 4, 5]);

        // To initialize all elements of an array with the same value,
        // you can write as follows:
        let arr2: [i32; 5] = [100; 5];
        assert_eq!(arr2, [100, 100, 100, 100, 100]);

        // You can create a tuple by arranging values of any type
        let tuple: (i32, &str, bool) = (1, "hello", true);
        // The elements of a tuple can be accessed by specifying the tuple variable followed by a dot and the index position.
        assert_eq!(tuple.0, 1);
        assert_eq!(tuple.1, "hello");
        assert_eq!(tuple.2, true);
    }
}
