#[cfg(test)]
mod tests {

    #[test]
    fn test_fn() {
        println!("a + b = {}", add(5, 8));
        show(format!("a + b = {}", add(5, 8)).as_str());
    }

    // 通常の関数定義では引数と戻り値の型を明示しなければならない
    fn add(a: i32, b: i32) -> i32 {
        // 関数の最後の置いた式が関数の戻り値となる。
        // a + bは式なのでセミコロンを置かないこと
        // セミコロンを置いた場合は文扱いになる。文は値を返さない(=`()`という型の値が暗黙で返される)
        a + b
        // 文で返したい場合はreturn文を使う
        // returnは文であるためセミコロンを使う。
        // return a + b; 
        //
        // returnは条件分岐の途中で関数を終了させる場合にも使う
    }

    // これは戻りの型を(a + bから推論できるはずだからと誤解して)省略した場合の例。
    // 省略した場合は戻りの型が`()`型(ユニット型)であると判断される。
    // ただし関数の最後の式はi32であり、これは`()`ではないため型が不一致となりビルドエラーとなる。
    // fn add(a: i32, b: i32) {
    //     a + b
    // }

    // この関数は値を返さないので戻りの型を省略してよい。
    // 関数の末尾に文が置かれており、`()`を返す関数として宣言されているため問題はない
    fn show(message: &str) {
        println!("{}", message);
    }

}