#[cfg(test)]
mod tests {

    #[test]
    fn test_ownership() {
        // 文字列リテラルは不変なので代入しても所有権の問題は発生しない
        let a = "hello";
        let b = a;

        // Stringは可変なので所有権の問題が発生する
        let x = String::from("world");
        let y = &x;
        println!("a: {}", a);
        println!("b: {}", b);
        println!("x: {}", x);
        println!("y: {}", y);
    }
}
