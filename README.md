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

## 遇到的问题
```shell
# 配置跨域之后启动失败
https://stackoverflow.com/questions/74867363/actix-cannot-start-server-service-0
```