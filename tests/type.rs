#[cfg(test)]
mod tests {

    use std::any::type_name;

    // この関数の型定義の意味は今はわからない...
    fn get_tuple_type_name<T: ?Sized + 'static>(_t: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn test_type() {
        // 同じ型同士では+で加算できる
        let a = 5 + 5;
        println!("a: {}", a);

        // 整数と浮動小数は型が違うためビルドエラーとなる
        // let b = 5 + 5.0;
        // println!("b: {}", b);

        // 整数と浮動小数型を加算するには整数をf32型にキャストしてから加算する
        let b = 5 as f32 + 5.0;
        println!("b: {:.1}", b);

        // 同じ整数型であってもi32とi64で加算することもできない
        //let c = 5 as i32 + 5 as i64
        //println!("c: {}", c);

        // タプル型の変数も定義できる
        let tuple = (1, "hello", true);
        let tuple_type_name = get_tuple_type_name(&tuple);
        println!("The type of tuple is: {}", tuple_type_name);
        // タプルの要素を参照するには`タプル変数.インデックス`でアクセスできる
        println!("The second item of tuple is: {}", tuple.1);

        // 配列の例
        let arr1 = [1,2,3,4,5];
        println!("len of arr1: {}", arr1.len());

        // 型名を指定する配列の例
        // 配列変数: [要素の型; 要素数]と宣言する
        let arr2: [i32; 5] = [1,2,3,4,5];
        println!("arr2: {:?}", arr2);

        // 全ての要素を同じ値で初期化する場合の例
        let arr3: [i32; 5] = [100; 5];
        println!("arr3: {:?}", arr3);
    }
}
