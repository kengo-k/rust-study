#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        password: String,
    }    

    fn create_user(username: String, email: String, password: String) -> User {
        User {
            username: username, 
            email: email,
            password: password
        }
    }

    #[test]
    fn test_struct() {
        let user = create_user("user1".to_string(), "email1".to_string(), "password1".to_string());
        println!("user: {:?}", user);
    }
}
