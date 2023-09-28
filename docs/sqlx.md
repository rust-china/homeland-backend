### sqlx

https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

##### 安装

Cargo.toml 文件里追加包

```
[dependencies]

sqlx = { version = "0.7", features = [
  "runtime-tokio-rustls",
  "postgres",
  "macros",
] }
```

```
# supports all databases supported by SQLx
$ cargo install sqlx-cli

# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres

# use vendored OpenSSL (build from source)
$ cargo install sqlx-cli --features openssl-vendored

# use Rustls rather than OpenSSL (be sure to add the features for the databases you intend to use!)
$ cargo install sqlx-cli --no-default-features --features rustls
```

##### 创建数据库

```bash
sqlx database create
```

##### 删除数据库

```bash
sqlx database drop
```

##### 生成迁移文件

```bash
sqlx migrate add -r create_user
```

##### 执行迁移文件

```bash
sqlx migrate run
```

##### 回滚上一次数据迁移

```bash
sqlx migrate revert
```
