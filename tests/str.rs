#[cfg(test)]
mod tests {

    #[test]
    fn test_print() {
        // 文字列リテラルの型は&strとなる
        let s: &str = "hello";
        println!("{}", s);

        // &strは変更できないので文字列を修正する場合はStringを使う
        let words = vec!["Hello", "World"];
        let mut result = String::new();
        for w in words {
            result.push_str(w);
        }
        println!("result string: {}", result)
    }
}
