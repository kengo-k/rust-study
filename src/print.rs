fn main() {
    // 通常の出力
    println!("Hello, world!");
    // {}を指定した箇所は後続の引数で置き換えられる
    println!("1 + 1 = {}", 1 + 1);
    // {0}や{1}などのように数値で位置を指定できる
    println!("0th value is {0}, 1th value is {1}, 0th value is {0}", 0, 1);
    // {key}などのように名前で位置を指定できる
    println!("{greet}, {name}!", greet="Hello", name="World");
  }
  