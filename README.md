# install rust

```
$ asdf plugin add rust
$ asdf list all rust
$ asdf install rust <rust-version>
$ asdf local rust <rust-version>
```

# install cargo-script

`cargo script`コマンドを使用できるようにする。`cargo script`を使用することで`go run`のようにバイナリを生成せずに直接main関数を持つファイルを実行できるようになる。`cargo-script`をインストールするには下記のコマンドを実行する。

```
$ cargo install cargo-script
```

インストール終了後、下記のように.rsファイルを指定してプログラムを実行できるようになる。

```
$ cargo script src/main.rs
Hello, world!
```

ただし`cargo-script`で実行できるのは単一のrustファイルの場合のみとなる。mod宣言を指定して他のファイルで定義された関数を呼び出している場合は`cargo-script`による実行はできない。代わりに`cargo run`を使用する。`cargo run`については後述。

# install cargo-edit

`cargo add`や`cargo rm`などのコマンドを使用できるようにする。これらのコマンドを使うことでCargo.tomlファイルを直接編集せずに自動的に依存関係を更新できるようになる(※NodeJSで言うところの`npm install`して自動的にpackage.jsonやpackage.lcokが更新されるイメージ)。

```
$ cargo install cargo-edit
```

# run main

`cargo run`を実行することでエントリポイントとなるrustファイルをビルドしてそのまま実行できる。ただしエントリポイントはデフォルトで`src/main.rs`となる。別の名前のrustファイルをエントリポイントとする場合は`Cargo.toml`にエントリポイントを追記する必要がある。例えば新たなエントリポイントとして`src/main2.rs`を使用する場合、下記のように`Cargo.toml`を修正する。

```
[[bin]]
name = "main2"
path = "src/main2.rs"
```

`Cargo.toml`を修正後、下記のように引数を指定して`cargo run`を実行する。

```
$ cargo run --bin main2
```

# run tests

```
$ cargo test
```

# パッケージとクレートについて

## パッケージとは

- パッケージ: 1つ以上のクレートの集合

パッケージはCargo.tomlファイルを持ち、Cargo.tomlが存在するディレクトリがパッケージのルートディレクトリとなる。Cargo.tomlファイルにはクレートの定義が記述されている。パッケージは一つ以上のクレートを持ち、パッケージがビルドの単位となる。

## クレートとは

- クレート: ライブラリおよび実行可能バイナリを表すもの。単語の意味としてはcrate=木箱、ケースを意味する。バイナリクレートを例にするとCargo.tomlの`[[bin]]`セクションが１クレートに相当する。