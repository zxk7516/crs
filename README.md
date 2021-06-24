## 安装手册

1. sudo apt inst postgresql
	```sh
	wget -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
	sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ focal-pgdg main" >> /etc/apt/sources.list.d/pgdg.list'
	sudo apt update
	sudo apt-get install postgresql postgresql-contrib
	sudo apt-get install libssl1.0-dev
	sudo apt-get install libssl-dev
	```

2. create mydb on postgres
	```
	# 切换到 postgres 用户
	me@ubuntu> sudo su postgres

	# 免密登录到 postgres 数据库最高账户
	postgres@ubuntu> psql

	# 创建用户 me(和linux 用户名相同，可以免密登录), 并赋予登录和创建数据库的权限
	postgres=# create role me CREATEDB Login;
	# 设置用户 me 的postgres 密码
	postgres=# alter user me password 'me_password';


	# Ctrl-D 登出 postgres 数据库
	# Ctrl-D 登出 postgres linux 用户

	# 创建数据库 mydb
	me@ubuntu> createdb mydb

	# 登录 mydb
	me@ubuntu> psql mydb
	```

3. 配置 dotenv 数据库连接

4. migration
	* 用 diesel, `diesel migration run`

	* 手动执行 sql 文件

5. Run
	```sh
	cargo build --release
	.target/release/crs
	```




