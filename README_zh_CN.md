# n-sql

[![Build Status](https://travis-ci.org/mokeyish/n-sql.svg?branch=master)](https://travis-ci.org/mokeyish/n-sql)
[![Build status](https://ci.appveyor.com/api/projects/status/ky63jhxhsq8rb2vy/branch/master?svg=true)](https://ci.appveyor.com/project/mokeyish/n-sql/branch/master)

[README](README.md) | [中文文档](README_zh_CN.md)

---

> **N-SQL是一个SQL解析，优化、翻译器和查询器**

- [n-sql](#n-sql)
  - [介绍](#%E4%BB%8B%E7%BB%8D)
  - [目标](#%E7%9B%AE%E6%A0%87)
  - [模块](#%E6%A8%A1%E5%9D%97)
  - [现状](#%E7%8E%B0%E7%8A%B6)
  - [例子](#%E4%BE%8B%E5%AD%90)
  - [参与](#%E5%8F%82%E4%B8%8E)

## 介绍

N-SQL是一个SQL语言，它提供简化且统一的SQL语言，能够将SQL翻译成Oracle、MySQL、PostgreSQL、Sql Server、SQLite等数据库支持的语言。例如分页，日期函数等。

## 目标

- 统一SQL查询语法，能够等效的翻译成目标数据库支持的SQL。
- 可以用SQL对Json、Excel、Csv等数据文件进行查询
- 对SQL进行优化
- 跨数据源查询

## 模块

 这是功能模块图
![modules](./docs/assets/misc/nsql_modules.png)

## 现状

- N-SQL目前还在开发中，它并不稳定。
- n-sql需要`Rust 1.34 +nightly`或者更新。

## 例子

- 你可以看当前项目的[单元测试](./tests) 或者在线**[试玩](https://n-sql.yish.org/playground/)**.

## 参与

想帮助我们开发`n-sql`？请看[`CONTRIBUTING.md`](./CONTRIBUTING.md)!