#[cfg(test)]
mod tests {

    #[test]
    fn test_struct() {
        // Create an instance of the struct.
        let john = Person {
            name: "John".to_string(),
            age: 20,
        };

        let name = "Alice".to_string();
        let alice = Person { name, age: 25 };

        // Create an instance of the struct using the associated function.
        let bob = Person::new("Bob".to_string(), 30);

        assert_eq!(john.name, "John");
        assert_eq!(john.age, 20);

        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.age, 25);

        assert_eq!(bob.name, "Bob");
        assert_eq!(bob.age, 30);
        assert_eq!(bob.greeting(), "Hello, I'm Bob, I am 30 years old");

        let p = Point(10, 20);
        assert_eq!(p.0, 10);
        assert_eq!(p.1, 20);
    }

    struct Person {
        name: String,
        age: i32,
    }

    // Define methods for the Person struct.
    impl Person {
        pub fn new(name: String, age: i32) -> Self {
            Person { name, age }
        }

        pub fn greeting(&self) -> String {
            format!("Hello, I'm {}, I am {} years old", &self.name, &self.age)
        }
    }

    // Tuple struct
    struct Point(i32, i32);
}
