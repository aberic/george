# client

客户端连接数据库命令如下：
```shell
george-client -H 127.0.0.1 -P 9219 -u admin -p admin#123
```
---
## 数据库系列命令
### 客户端初始化状态命令如下：
```shell
clear;
```
---

### 显示客户端状态命令如下：
```shell
show;
```
### 显示数据库列表命令如下：
```shell
show databases;
```
### 创建数据库命令如下：
```shell
create database [database:string];
```
或
```shell
create database [database:string] [comment:string];
```
### 显示数据库信息命令如下：
```shell
inspect database [database:string];
```
### 修改数据库命令如下：
```shell
alter database [database:string] [database:string];
```
或
```shell
alter database [database:string] [database:string] [comment:string];
```
### 删除数据库命令如下：
```shell
drop database [database:string];
```
---

## 缓存页系列命令
### 显示缓存页列表命令如下：
```shell
show pages;
```
### 创建缓存页命令如下：
```shell
create page [page:string];
```
或
```shell
create page [page:string] [comment:string];
```
### 显示缓存页信息命令如下：
```shell
inspect page [page:string];
```
### 修改缓存页命令如下：
```shell
alter page [page:string] [page:string];
```
### 删除缓存页命令如下：
```shell
drop page [page:string];
```
---

## 视图系列命令
### 显示视图列表命令如下：
```shell
show views;
```
### 显示视图信息命令如下：
```shell
inspect view from [database:string];
```
### 创建视图命令如下：
```shell
create view [view:string] [increment:bool];
```
或
```shell
create view [view:string] [increment:bool] [comment:string];
```
### 显示视图信息命令如下：
```shell
inspect view [view];
```
### 修改视图命令如下：
```shell
alter view [view:string] [view:string];
```
或
```shell
alter view [view:string] [view:string] [comment:string];
```
### 删除视图命令如下：
```shell
drop view [view:string];
```
### 视图归档命令如下：
```shell
alter archive [view:string] [filepath:String];
```
### 读取指定版本视图归档信息命令如下：
```shell
show record [view:string] [version:u16];
```
### 读取所有视图归档信息命令如下：
```shell
show records [view:string];
```
---

## 索引系列命令
### 显示索引列表命令如下：
```shell
show indexes from [view:string];
```
### 显示索引信息命令如下：
```shell
inspect index [index:string] from [view:string];
```
### 创建索引命令如下：
```shell
create index [index:string] from [view:string] [primary:bool] [unique:bool] [null:bool] [key_type:string] [engine:string];
```
或
```shell
create index [index:string] from [view:string];
```
---

## 数据内存系列命令
### 插入数据命令如下：
```shell
put [key:string] [value:string];
```
或
```shell
put [key:string] [value:string];
```
### 修改或插入数据命令如下：
```shell
set [key:string] [value:string];
```
或
```shell
set [key:string] [value:string];
```
### 获取数据命令如下：
```shell
get [key:string];
```
或
```shell
get [key:string];
```
### 删除数据命令如下：
```shell
remove [key:string];
```
或
```shell
remove [key:string];
```
---

## 数据磁盘系列命令
### 插入数据命令如下：
```shell
put [view:string] [key:string] [value:string];
```
或
```shell
put [view:string] [key:string] [value:string];
```
### 修改或插入数据命令如下：
```shell
set [view:string] [key:string] [value:string];
```
或
```shell
set [view:string] [key:string] [value:string];
```
### 获取数据命令如下：
```shell
get [view:string] [key:string];
```
或
```shell
get [view:string] [key:string] [index:string];
```
或
```shell
get [view:string] [key:string];
```
或
```shell
get [view:string] [key:string] [index:string];
```
### 删除数据命令如下：
```shell
remove [view:string] [key:string];
```
或
```shell
remove [view:string] [key:string];
```
### 富操作查询数据命令如下：
```shell
select [view:string] [constraint:string];
```
### 富操作删除数据命令如下：
```shell
delete [view:string] [constraint:string];
```
