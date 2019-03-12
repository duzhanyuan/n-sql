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

extern crate n_sql;

use n_sql::{ExpressionParser, Lexer};


#[theory]
#[test]
#[case("day(now())", NSQL, "day(now())")]
#[case("extract(day from now())", NSQL, "day(now())")]
#[case("day(now())", PostgreSQL, "extract(day from now())")]

fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("day_add(now(),3)", NSQL, "day_add(now(), 3)")]
#[case("day_add(now(),3)", PostgreSQL, "now() + interval '3' day")]
#[case("day_add(now(),3+9*7)", NSQL, "day_add(now(), 3 + 9 * 7)")]
#[case("day_add(now(),3+9*7)", PostgreSQL, "now() + interval '66' day")]
//#[case("day_add(now(),3+9*7)", Oracle, "systimestamp + interval '66' day")]
//#[case("day_add(now(),3+9*7)", MySQL, "date_add(now(), interval 66 day)")]
//#[case("day_add(now(),3+9*7)", SQLite, "Datetime(Current_Timestamp, '+66 Day')")]
fn test_day_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("day_sub(now(),3)", NSQL, "day_sub(now(), 3)")]
#[case("day_sub(now(),3)", PostgreSQL, "now() - interval '3' day")]
#[case("day_sub(create_date,3)", NSQL, "day_sub(create_date, 3)")]
fn test_day_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("HOUR_add(now(),3)", NSQL, "hour_add(now(), 3)")]
#[case("hour_add(now(),3)", PostgreSQL, "now() + interval '3' hour")]
fn test_hour_add(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[theory]
#[test]
#[case("HOUR_sub(now(),3)", NSQL, "hour_sub(now(), 3)")]
#[case("HOUR_sub(now(),3)", PostgreSQL, "now() - interval '3' hour")]
fn test_hour_sub(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("HOUR(now())", NSQL, "hour(now())")]
#[case("HOUR(now())", PostgreSQL, "extract(hour from now())")]
#[case("extract(hour from now())", NSQL, "hour(now())")]
#[case("extract(hour from now())", PostgreSQL, "extract(hour from now())")]
fn test_hour(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[theory]
#[test]
#[case("minute_add(now(),3)", NSQL, "minute_add(now(), 3)")]
#[case("minute_add(now(),3)", PostgreSQL, "now() + interval '3' minute")]
fn test_minute_add(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}



#[theory]
#[test]
#[case("minute_sub(now(),3)", NSQL, "minute_sub(now(), 3)")]
#[case("minute_sub(now(),3)", PostgreSQL, "now() - interval '3' minute")]
fn test_minute_sub(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[theory]
#[test]
#[case("minute(now())", NSQL, "minute(now())")]
#[case("minute(now())", PostgreSQL, "extract(minute from now())")]
#[case("extract(minute from now())", NSQL, "minute(now())")]
#[case("extract(minute from now())", PostgreSQL, "extract(minute from now())")]
fn test_minute(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month_add(now(),3)", NSQL, "month_add(now(), 3)")]
#[case("month_add(now(),3)", PostgreSQL, "now() + interval '3' month")]
//#[case("month_add(now(),3)", Oracle, "systimestamp + interval '3' month")]
//#[case("month_add(now(),3)", MySQL, "date_add(now(), interval 1 month)")]
//#[case("month_add(now(),3)", SQLite, "datetime(current_timestamp, '-3 Second')")]
fn test_month_add(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month_sub(now(),3)", NSQL, "month_sub(now(), 3)")]
#[case("month_sub(now(),3)", PostgreSQL, "now() - interval '3' month")]
fn test_month_sub(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[theory]
#[test]
#[case("month(now())", NSQL, "month(now())")]
#[case("month(now())", PostgreSQL, "extract(month from now())")]
//#[case("month(now())", Oracle, "extract(month from systimestamp)")]
#[case("extract(month from now())", NSQL, "month(now())")]
#[case("extract(month from now())", PostgreSQL, "extract(month from now())")]
fn test_month(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[case("second_add(now(),3)", NSQL, "second_add(now(), 3)")]
#[case("second_add(now(),3)", PostgreSQL, "now() + interval '3' second")]
fn test_second_add(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("second_sub(now(),3)", NSQL, "second_sub(now(), 3)")]
#[case("second_sub(now(),3)", PostgreSQL, "now() - interval '3' second")]
fn test_second_sub(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("second(now())", NSQL, "second(now())")]
#[case("second(now())", PostgreSQL, "extract(second from now())")]
#[case("extract(second from now())", NSQL, "second(now())")]
#[case("extract(second from now())", PostgreSQL, "extract(second from now())")]
fn test_second(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year_add(now(),3)", NSQL, "year_add(now(), 3)")]
#[case("year_add(now(),3)", PostgreSQL, "now() + interval '3' year")]
//#[case("year_add(now(),3)", Oracle, "systimestamp + interval '3' year")]
//#[case("year_add(now(),3)", MySQL, "date_add(now(), interval 3 year)")]
//#[case("year_add(now(),3)", SQLite, "datetime(current_timestamp, '-3 second')")]
fn test_year_add(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year_sub(now(),3)", NSQL, "year_sub(now(), 3)")]
#[case("year_sub(now(),3)", PostgreSQL, "now() - interval '3' year")]
fn test_year_sub(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[theory]
#[test]
#[case("year(now())", NSQL, "year(now())")]
#[case("year(now())", PostgreSQL, "extract(year from now())")]
//#[case("year(now())", Oracle, "extract(year from systimestamp)")]
#[case("extract(year from abc)", NSQL, "year(abc)")]
#[case("extract(year from abc)", PostgreSQL, "extract(year from abc)")]
fn test_year(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}


#[test]
fn test_year_err(){
    let expr = ExpressionParser::new().parse(Lexer::new("extract(year form abc)").tokenizer());
    assert_eq!(true, expr.is_err());
}