#[cfg(test)]
mod tests {

    #[test]
    fn test_var() {
        // letで変数を定義する
        let x = 5;
        println!("1st x: {}", x);
        
        // letで定義した変数は変更できないため下記のコードはビルドエラーとなる。
        // x = 10;
        // 再度letで定義した場合は前の定義を上書きするので問題ない
        let x = 10;
        println!("2nd x: {}", x);
        
        // 変更可能な変数を定義するためにはletのあとにmutを指定する
        let mut y = 20;
        println!("1st y: {}", y);
        y = 30;
        println!("2nd y: {}", y);
    }
}
