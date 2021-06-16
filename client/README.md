# client

客户端连接数据库命令如下：
```shell
client -H 127.0.0.1 -P 9219 -u admin -p admin#123
```
---
### 数据库系列命令
#### 显示数据库列表命令如下：
```shell
show databases;
```

#### 创建数据库命令如下：
```shell
create database test1;
```
或
```shell
create database test1 tt1;
```

#### 显示数据库信息命令如下：
```shell
info database sys;
```
---

### 缓存页系列命令
#### 显示缓存页列表命令如下：
```shell
show pages;
```

#### 创建缓存页命令如下：
```shell
create page test1;
```
或
```shell
create page test1 tt1;
```

#### 显示缓存页信息命令如下：
```shell
info page sys;
```
---

### 视图系列命令
#### 显示视图列表命令如下：
```shell
show views;
```
---

### 视图系列命令
#### 显示视图列表命令如下：
```shell
show views;
```
---

### 索引系列命令
#### 显示索引列表命令如下：
```shell
show indexes from [view];
```
