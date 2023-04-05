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
$ cargo script src/hello.rs 
Hello, world!
```

# install cargo-edit

`cargo add`や`cargo rm`などのコマンドを使用できるようにする。これらのコマンドを使うことでCargo.tomlファイルを直接編集せずに自動的に依存関係を更新できるようになる(※NodeJSで言うところの`npm install`して自動的にpackage.jsonやpackage.lcokが更新されるイメージ)。

```
$ cargo install cargo-edit
```