fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // letで宣言した変数は不変のため下記のように代入して変更することはできない
    // x = 100; // これはビルドエラーになる
    // 変更可能な変数を宣言するためには`mut`を指定する
    // この例では同名の変数xを宣言しシャドーイングしている
    let mut x = 100;
    println!("The value of x is: {}", x);
    x = 200; // これはビルドできる
    println!("The value of x is: {}", x);

    println!("The const of X is: {}", X);
}

// constは定数を宣言する
// mutなしの変数の違いは
// - 型を必ず指定しなければならない
// - 固定値を指定しなければならない(関数の戻り値をセットすることはできない)
const X: u32 = 1000;