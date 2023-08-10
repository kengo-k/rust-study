#[cfg(test)]
mod tests {

    #[derive(Debug)]
    pub struct Person {
        pub age: i32,
        pub name: String,
    }

    impl Person {
        pub fn new(age: i32, name: String) -> Person {
            Person { age, name }
        }

        pub fn set(&mut self, age: i32, name: String) {
            self.age = age;
            self.name = name;
        }
    }

    #[test]
    fn test_var1() {
        // Declare a variable using 'let'.
        let x = 5;
        println!("1st x: {}", x);

        // Variables declared with 'let' are immutable, so the code below will not compile.
        // x = 10;

        // You can declare the same variable name again using 'let'
        let x = 10;
        println!("2nd x: {}", x);

        // By declaring a variable with 'mut', you can change its value.
        let mut y = 10;
        println!("1st y: {}", y);

        y = 20;
        println!("2nd y: {}", y);
    }

    #[test]
    #[allow(unused_variables)]
    fn test_var2() {
        // In the case of a mutable struct, both reassignment to the variable and modification of the struct's values become possible.
        let mut p1 = Person::new(30, "John".to_string());

        // reassignment
        p1 = Person::new(35, "Bob".to_string());

        // update struct's values
        //
        // Since the variable p1 is the owner, it is possible to call a method where the receiver is a mutable reference.
        p1.set(32, "Alice".to_string());

        println!("person 1st: {:?}", p1);

        // It is possible to modify the value of a struct through a mutable reference.
        // In this case, the 'mut' keyword is not required in the variable declaration.
        let rp1 = &mut p1;
        rp1.set(40, "Scott".to_string());
        println!("person 2nd: {:?}", rp1);

        // The variable rp1 does not have 'mut' specified, so it cannot be reassigned.
        // So the code below will not compile.
        // rp1 = &mut p1;

        // redclare rp1 using let without 'mut'
        let rp1 = &p1;
        // rp1 is not mutable, so it cannot be modified and the code below will not compile.
        //rp1.set(40, "Scott".to_string());
    }
}
