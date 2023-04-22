#[cfg(test)]
mod tests {

    #[test]
    fn test_ownership() {
        let a = String::from("hello");
        let b = a;
        println!("a: {}", a);
        println!("b: {}", b);
    }
}
