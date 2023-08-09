#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter, Result};

    struct Person {
        name: String,
        age: i32,
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct DebuggablePerson {
        name: String,
        age: i32,
    }

    // Implementing the Display trait
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[test]
    fn test_print() {
        // Use println! without arguments.
        println!("Hello, world!");

        // By specifying {} within the output string, that part can be replaced with the value of the subsequent arguments.
        println!("1 + 1 = {}", 1 + 1);

        // In the output string, you can specify a position using the format {n}, where n is the index of the argument.
        // This will replace that part of the string with the value of the corresponding argument in the provided order.
        println!("0th value is {0}, 1th value is {1}, 0th value is {0}", 0, 1);

        // In the output string, you can use placeholders in the {key} format, where key corresponds to the name of an argument.
        // This will replace that part of the string with the value associated with the specified key.
        println!("{greet}, {name}!", greet = "Hello", name = "World");

        // In order to output with {}, the Display trait must be implemented.
        // Since Vec<i32> does not implement the Display trait, it results in a build error
        //let values = vec![1,2,3];
        //println!("values: {}", values);

        // Vec<i32> does not implement the Display trait,
        // but since it implements the Debug trait, it can be output using {:?}.
        let values = vec![1, 2, 3];
        println!("values: {:?}", values);

        // User-defined structs that do not implement the Debug or Display traits cannot be output using println!
        //let john = Person { name: "John".to_string(), age: 20};
        //println!("person: {}", john);

        // By implementing the Display trait for a user-defined struct,
        // it becomes possible to output it using {}.
        let p1 = Point { x: 5, y: 10 };
        println!("point: {}", p1);

        // When implementing the Debug trait,
        // it can be achieved without manual implementation by using the derive attribute.
        let bob = DebuggablePerson {
            name: "Bob".to_string(),
            age: 25,
        };
        println!("person: {:?}", bob);
    }
}
