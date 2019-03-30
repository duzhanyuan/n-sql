# n-sql

[![Build Status](https://travis-ci.org/mokeyish/n-sql.svg?branch=master)](https://travis-ci.org/mokeyish/n-sql)
[![Build status](https://ci.appveyor.com/api/projects/status/ky63jhxhsq8rb2vy/branch/master?svg=true)](https://ci.appveyor.com/project/mokeyish/n-sql/branch/master)

[README](README.md) | [中文文档](README_zh_CN.md)

---

> **N-SQL is a SQL statement parser, optimizer, generator and Queryer.**


- [n-sql](#n-sql)
  - [What](#what)
  - [Goals](#goals)
  - [Modules](#modules)
  - [Status](#status)
  - [Example](#example)
  - [Get Involved！](#get-involved)

## What

N-SQL is a SQL language that provides a simplified and unified SQL language that translates SQL into languages ​​supported by databases such as Oracle, MySQL, PostgreSQL, Sql Server, and SQLite. For example, pagination, date functions, etc.

## Goals

- The unified SQL query syntax can be equivalently translated into SQL supported by the target database.
- Query Json, Excel, Csv and other data files with n-sql.
- Optimize SQL
- Cross-data source query

## Modules

This is the graph of functional module.
![modules](./docs/assets/misc/nsql_modules.png)

## Status

N-SQL currently is still under development. It is not yet stable.

N-SQL currently requires Rust 1.33 stable or later

## Example

- You could check out the [unit tests](./tests) or play **[online](https://n-sql.yish.org/playground/)**.

## Get Involved！

Want to help us build `n-sql`? Check out [`CONTRIBUTING.md`](./CONTRIBUTING.md)!
