#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        password: String,
    }    

    struct User2<'a> {
        username: &'a str
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
        let name2 = "john";
        let user2 = User2 {
            username: name2
        };
        println!("user: {:?}", user);
    }
}
