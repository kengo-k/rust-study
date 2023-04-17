#[cfg(test)]
mod tests {

    #[test]
    fn test_print() {
        let message = get_message("World");
        println!("{}", message);
    }

    fn get_message(name: &str) -> String {
        return format!("Hello, {}!", name);
    }
}
