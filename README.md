# caenv

[Cloud AutomatorのCLIツール](https://blog.serverworks.co.jp/cloudautomator-cli)で利用する環境変数(CLOUDAUTOMATOR_API_KEY)を管理するツールです。

## インストール

### 前提
ツールのビルドにcargo が必要です

### 手順

1. 本リポジトリを git clone し、実行パスに通してください。
2. Rust スクリプトをビルドしてください。
```
cargo build
```

## 使い方

`~/.cloudautomator/credentials` にAPIキーを保存してください。
形式は toml 形式となっており、以下のように記述します。

```sh
[production]
api_key = 11111111111111111111111111111111
[qa]
api_key = 22222222222222222222222222222222
[dev]
api_key = 33333333333333333333333333333333
```

本リポジトリの `cargo` を `source` コマンドで実行します。

```sh
source ./caenv
```

あらかじめエイリアスに登録しておくと便利でしょう。

```sh
alias caenv = "source caenv"
```
