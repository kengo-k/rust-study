#[cfg(test)]
mod tests {
    
    struct User {
        name: String,
        email: String,
        password: String,
    }   

    #[derive(Debug)]
    struct DebuggableUser {
        name: String,
        email: String,
        password: String,
    }

    #[derive(Debug)]
    struct Point(i32, i32);

    // fn create_user(username: String, email: String, password: String) -> User {
    //     User {
    //         username: username, 
    //         // JavaScriptと同様に変数名とフィールド名が一致する場合は省略して記述することができる
    //         email,
    //         password
    //     }
    // }

    #[test]
    fn test_struct() {
        let user = User {
            name: "john".to_string(),
            email: "john@example.com".to_string(),
            password: "xxx".to_string(),
        };

        // {}を指定して構造体を出力してもエラーとなる。
        // {}は指定された値(ここでは変数user)がDisplayトレイトを実装していることを前提とするため
        // → Displayトレイトに定義されているfmtメソッドが呼び出される
        //println!("user: {}", user);

        // {}を{:?}に変更するとDisplayではなくDebugトレイトを実装していることを要求される。
        // ※ User構造体はDebugトレイトも実装していないため上記と同様にエラーとなる。
        // → DebugトレイトにもDisplayと同様にfmtメソッドが定義されている
        //println!("user: {:?}", user);

        let debuggable_user = DebuggableUser {
            name: "john".to_string(),
            email: "john@example.com".to_string(),
            password: "xxx".to_string(),
        };
        println!("debuggable_user: {:?}", debuggable_user);

        // let name2 = "john";
        // let user2 = User2 {
        //     username: name2
        // };
        // let user3 = User {
        //     username: "yamada".to_string(),
        //     ..user
        // };
        // println!("user3: {:?}", user3);

        // let point = Point(0, 0);
        // println!("point: {:?}", point);
    }
}
