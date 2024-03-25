# short-link

短链接服务端

[English readme](./README.md)

## 编译

为默认平台编译: `cargo build --release`

为 Linux (musl) 平台编译: `./build.sh`

## 怎么用

1. 先编译, 然后把编译好的程序 `short_link` 和 `ShortLink.toml` 复制到部署环境

2. 安装 `postgresql`, 创建一个用户和数据库:

```bash
$ sudo -u postgres psql
psql (15.6 (Debian 15.6-0+deb12u1))
Type "help" for help.

postgres=# create user 用户名 password '数据库密码';
CREATE ROLE
postgres=# create database 数据库名 owner 用户名;
CREATE DATABASE
```

3. 修改 `ShortLink.toml`:

```toml
database_url = "postgres://用户名:数据库密码@127.0.0.1/数据库名"
host = "127.0.0.1"  # 服务端地址 `http://127.0.0.1`
port = 3000  # 服务端端口 `3000`
base = "/"  # 所有路由基于 `/`
...
[service]
secret = "管理员密码"
...
```

4. 运行 `short_link`

5. [看看管理员 API 示例](./example_client.py)

## 许可证

Apache 2.0

```text
Copyright 2024 @sb-child

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
