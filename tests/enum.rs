#[cfg(test)]
mod tests {

    // 列挙型の例
    enum Status {
        SUCCESS,
        // 個々の列挙型の値は独自の値を持つことができる
        // ここでは単純な数値型やStringを使用しているが構造体や列挙型など任意の値を持てる
        ERROR(i32, String)
    }

    #[test]
    fn test_enum() {
        let s1 = Status::SUCCESS;
        let s2 = Status::ERROR(404, "NOT FOUND".to_string());

        fn check_status(s: Status) -> bool {
            match s {
                Status::SUCCESS => true,
                // マッチが不要な変数は_を使用できる
                Status::ERROR(_, msg) => {
                    println!("error: {}", msg);
                    false
                }
            }
        }

        check_status(s1);
        check_status(s2);
    }

}
