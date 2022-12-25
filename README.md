## 依赖库
```shell
# actix-web
https://github.com/actix/actix-web

# sqlx
https://crates.io/crates/sqlx
```

## 配置
```shell
# 配置目录
config/Settings.toml

## 内容如下
[server]
ip = "0.0.0.0"
port = 8000

[database]
url = "mysql://<username>:<password>@<address>/<database>"
pool_size = 5
```