#[cfg(test)]
mod tests {
    
    struct User {
        name: String,
        email: String,
        password: String,
        age: i32,
    }

    // 構造体Userにメソッドgreetingを定義する例
    impl User {
        fn greeting(&self) -> String {
            format!("Hello, My name is {}, and I am {} years old", self.name, self.age)
        }
    }

    #[derive(Debug)]
    struct DebuggableUser {
        name: String,
        email: String,
        password: String,
        age: i32,
    }

    // タプル構造体
    // フィールド名を持つほどでもない簡単な構造体に使える
    #[derive(Debug)]
    struct Point(i32, i32);

    #[test]
    fn test_struct() {
        let age = 25;
        let john = User {
            name: "john".to_string(),
            email: "john@example.com".to_string(),
            password: "xxx".to_string(),
            // フィールド名と値が同じ場合は省略記法が使える(JSのObjectと同様の機能)
            age,
        };

        println!("john's greeting: {}", john.greeting());

        // {}を指定して構造体を出力してもエラーとなる。
        // {}は指定された値(ここでは変数user)がDisplayトレイトを実装していることを前提とするため
        // → Displayトレイトに定義されているfmtメソッドが呼び出される
        //println!("john: {}", john);

        // {}を{:?}に変更するとDisplayではなくDebugトレイトを実装していることを要求される。
        // ※ User構造体はDebugトレイトも実装していないため上記と同様にエラーとなる。
        // → DebugトレイトにもDisplayと同様にfmtメソッドが定義されている
        //println!("john: {:?}", john);

        let debuggable_john = DebuggableUser {
            name: "john".to_string(),
            email: "john@example.com".to_string(),
            password: "xxx".to_string(),
            age,
        };
        println!("debuggable_john: {:?}", debuggable_john);

        let debuggable_bob = DebuggableUser {
            name: "bob".to_string(),
            email: "bob@example.com".to_string(),
            // ..を使うと既存の構造体のフィールドを展開して初期化に使うことができる
            // ここではnameとemailはすでに指定されているのでpasswordとageが展開される
            ..debuggable_john
        };
        println!("debuggable_bob: {:?}", debuggable_bob);


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
