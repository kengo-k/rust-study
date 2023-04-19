#[cfg(test)]
mod tests {

    #[test]
    fn test_print() {
        // 文字列リテラルの型は&strとなる
        let s: &str = "hello";
        println!("{}", s);

        // &strは変更できないので文字列を修正する場合はStringを使う
        let words = ["Hello", ",", "World", "!"];
        // 変数がresultが指すオブジェクトは変えていない（=再代入はしていない)が、
        // (pust_strにより)オブジェクトの中身に修正を加えているのでmut指定が必要。
        // ※JSなど他の言語ではconst変数に代入したarrayにpushできる点で仕様が異なっている
        let mut result = String::new();
        // 変数wをpush_strに渡すためには所有権が必要になるため&で借用する。
        // for文が終了しスコープを抜けた時点で借用は終了する。
        for w in words {
            result.push_str(w);
        }
        println!("result string: {}", result)
    }
}
