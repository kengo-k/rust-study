# install rust

```
asdf plugin add rust
asdf list all rust
asdf install rust <rust-version>
asdf local rust <rust-version>
```

# install cargo-edit

`cargo add`コマンドを使用できるようにする`cargo-edit`をインストールする。
`cargo-edit`はグローバルインストールされる。以後、`cargo add <crate-name>`でプロジェクトで利用するクレートをインストールできるようになる。インストールされたクレートはCargo.tomlとCargo.lockファイルに記載される。