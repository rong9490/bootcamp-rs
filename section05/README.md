```
  2025-09-07
  5-5
  https://u.geekbang.org/lesson/610?article=779626
  
```

```
  5-5 https://u.geekbang.org/lesson/610?article=779626 30:09
  cargo add tokio --features rt --features rt-multi-thread --features macros --features net
  cargo add axum --features http2 --features query --features tracing --features multipart
  cargo add sqlx --features postgres --features runtime-tokio-rustls
  cargo add serde --features derive
  cargo add anyhow / thiserror / serde-yaml
```

```
  HTTP协议入门 (rfc2616阅读)
  https://u.geekbang.org/lesson/610?article=779589

  组成 / 语义 / 

  IOS 7层结构 / DNS server / TLS 加解密 / web proxy / user-agent / session-id

  HTTP flow / cache / restful API

```

```
  初识axum
  https://u.geekbang.org/lesson/610?article=779610

  tokio -> request -> axum

  matchit 路由匹配

  to response...
```

```
  开始搭建项目:
  https://u.geekbang.org/lesson/610?article=779626
  tokio / axum / anyhow / thiserror / sqlx
  
  https://u.geekbang.org/lesson/610?article=779656
  cargo install sqlx-cli --no-default-features --features rustls --features postgres
  sqlx命令: database / prepare / migrate / help
  
  postgres: / psql / dropdb / createdb / pgcli
  created chat
  pgcli python写的工具
  sqlx migrate add initial 初始化
  sqlx migrate run  需要环境变量数据库地址
	
	select * from _sqlx_migrations;
	会有version版本划分;
	
	cargo install xx -p 指定子项目安装依赖
```
