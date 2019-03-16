// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod common;

test_init!();

#[theory]
#[test]
#[case("nvl(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("nvl(a, 'no value')", MySQL, "ifnull(a, 'no value')")]
#[case("nvl(a, 'no value')", PostgreSQL, "coalesce(a, 'no value')")]
#[case("NVL(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("NVL(a,      123)", NSQL, "nvl(a, 123)")]
#[case("NVL(a,      -123)", NSQL, "nvl(a, -123)")]
#[case("nvl (abc, 'none')", SqlServer, "coalesce(abc, 'none')")]
#[case("nvl (abc, 'none')", SQLite, "ifnull(abc, 'none')")]

fn test_nvl(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("COALESCE(a,b,c)", NSQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", PostgreSQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", Oracle, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", MySQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", SqlServer, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", SQLite, "coalesce(a, b, c)")]
fn test_coalesce(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
//#[case("format(now(), 'yyyy-mm-dd')", NSQL, "format(now(), 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", PostgreSQL, "to_char(now(), 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", Oracle, "to_char(systimestamp, 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", MySQL, "date_format(now(), '%Y-%m-%d')")]
//
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", NSQL, "format(now(), 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", MySQL, "date_format(now(), '%Y-%m-%d %H:%i:%s')")]
//
//
//#[case("format(now(), 'yyyy/mm/dd')", NSQL, "format(now(), 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", PostgreSQL, "to_char(now(), 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", Oracle, "to_char(systimestamp, 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", MySQL, "date_format(now(), '%Y/%m/%d')")]
//
//
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", NSQL, "format(now(), 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", MySQL, "date_format(now(), '%Y/%m/%d %H:%i:%s')")]
//
//
//
//#[case("format(now(), 'yyyy年mm月dd日')", NSQL, "format(now(), 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", PostgreSQL, "to_char(now(), 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", Oracle, "to_char(systimestamp, 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", MySQL, "date_format(now(), '%Y年%m月%d日')")]
//
//
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", NSQL, "format(now(), 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", MySQL, "date_format(now(), '%Y年%m月%d日 %H:%i:%s')")]
#[allow(dead_code)]
fn test_format(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}
