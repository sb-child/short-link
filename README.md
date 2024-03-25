# short-link

Short link server

[中文版](./README-zhcn.md)

## Build

build for your default target: `cargo build --release`

build for Linux (musl): `./build.sh`

## Usage

1. Build this program and copy the release executable `short_link` and `ShortLink.toml` to your server.

2. Install `postgresql` and create a user and database:

```bash
$ sudo -u postgres psql
psql (15.6 (Debian 15.6-0+deb12u1))
Type "help" for help.

postgres=# create user USER_NAME password 'STRONG_PASSWORD';
CREATE ROLE
postgres=# create database DATABASE_NAME owner USER_NAME;
CREATE DATABASE
```

3. Modify your config file `ShortLink.toml`:

```toml
database_url = "postgres://USER_NAME:STRONG_PASSWORD@127.0.0.1/DATABASE_NAME"
host = "127.0.0.1"  # this short_link server will listen on `http://127.0.0.1`
port = 3000  # and the port will be `3000`
base = "/"  # all routes will be based on `/`
...
[service]
secret = "ANOTHER_STRONG_PASSWORD"
...
```

4. Run the `short_link` executable.

5. [Refer the Admin API example](./example_client.py)

## License

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
