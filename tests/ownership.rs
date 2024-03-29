#[cfg(test)]
mod tests {

    #[derive(Clone, Copy)]
    struct Struct1 {
        i: i32,
    }

    // deriveを指定しない構造体を定義
    struct Struct2 {
        i: i32
    }

    #[allow(unused_variables)]
    #[test]
    fn test_ownership() {
        // 数値型の代入は単純なコピーとなるため所有権の問題が発生する
        let a = 10;
        let b = a;
        println!("a: {}", a);
        println!("b: {}", b);

        // Stringは内部に可変のデータ(ヒープ領域)を持つ。
        // xをyに代入するとxが保持するヒープ領域のポインタがyにコピーされる。
        // するとxとyはヒープ領域の同じアドレスを参照するポインタを持つ。
        //
        // このときyが指すヒープ領域が解放された場合xが指すヒープ領域も
        // 同時に無効になってしまう。その後に意図せず変数xを参照すると
        // メモリエラーが発生する。

        // これを防ぐため所有権を持つ変数がただ一つであることをコンパイラが強制する。
        // この仕組みを所有権という。
        // 先のxとyの例で言うとxをyに代入した時点で所有権はyに移動するため
        // xへアクセスすることはできなくなる。

        // 例えば以下のコードはビルドエラーになる。
        // let x = String::from("world");
        // let y = x;
        // println!("x: {}", x);

        // xに所有権を残したまま変数yも参照可能にするには&をつけて所有権を借用する
        let x = String::from("world");
        let y = &x;
        println!("x: {}", x);
        println!("y: {}", y);

        // 構造体の変数st1をst2にコピーする場合の例
        // 構造体Struct1はCopyトレイトを実装しているため
        // 代入を行うとコピーが行われるため所有権のムーブは起きない
        let st1 = Struct1 {
            i: 5,
        };

        // &をつけて借用しなくてもst2を参照できる
        let st2 = st1;
        println!("st1[i]: {}", st1.i);

        let st3 = Struct2 {
            i: 5,
        };

        let st4 = st3;
        // st3は所有権をst4に譲渡したのでst3を参照することはできない
        //println!("st3[i]: {}", st3.i);
        println!("st4[i]: {}", st4.i);

    }
}
