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
#[case("day(now())", NSQL, "day(now())")]
#[case("extract(day from now())", NSQL, "day(now())")]
#[case("day(now())", PostgreSQL, "extract(day from now())")]

#[case("day_add(now(),3)", NSQL, "day_add(now(), 3)")]
#[case("day_add(now(),3)", PostgreSQL, "now() + interval '3' day")]

#[case("day_add(now(),3+9*7)", NSQL, "day_add(now(), 3 + 9 * 7)")]
#[case("day_add(now(),3+9*7)", PostgreSQL, "now() + interval '66' day")]


fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}


#[theory]
#[case("day_sub(now(),3)", NSQL, "day_sub(now(), 3)")]
#[case("day_sub(now(),3)", PostgreSQL, "now() - interval '3' day")]
fn test_day_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}



#[test]
fn test_day_sub1(){
    test_expression(NSQL, "day_sub(create_date,3)", "day_sub(create_date, 3)");
}

#[test]
fn test_hour_add(){
    test_expression(NSQL,"HOUR_add(now(),3)", "hour_add(now(), 3)");
}


#[test]
fn test_pgsql_hour_add(){
    test_expression(PostgreSQL,"hour_add(now(),3)", "now() + interval '3' hour");
}


#[test]
fn test_hour_sub(){
    test_expression(NSQL, "HOUR_sub(now(),3)", "hour_sub(now(), 3)");
}

#[test]
fn test_pgsql_hour_sub(){
    test_expression(PostgreSQL,"HOUR_sub(now(),3)", "now() - interval '3' hour");
}


#[test]
fn test_hour(){
    test_expression(NSQL, "HOUR(now())", "hour(now())");
}
#[test]
fn test_pgsql_hour(){
    test_expression(PostgreSQL,"HOUR(now())", "extract(hour from now())");
}
#[test]
fn test_hour1(){
    test_expression(NSQL, "extract(hour from now())", "hour(now())");
}
#[test]
fn test_pgsql_hour1(){
    test_expression(PostgreSQL,"extract(hour from now())", "extract(hour from now())");
}

#[test]
fn test_minute_add(){
    test_expression(NSQL, "minute_add(now(),3)", "minute_add(now(), 3)");
}

#[test]
fn test_pgsql_minute_add(){
    test_expression(PostgreSQL,"minute_add(now(),3)", "now() + interval '3' minute");
}

#[test]
fn test_minute_sub(){
    test_expression(NSQL, "minute_sub(now(),3)", "minute_sub(now(), 3)");
}

#[test]
fn test_pgsql_minute_sub(){
    test_expression(PostgreSQL,"minute_sub(now(),3)", "now() - interval '3' minute");
}

#[test]
fn test_minute(){
    test_expression(NSQL, "minute(now())", "minute(now())");
}
#[test]
fn test_pgsql_minute(){
    test_expression(PostgreSQL,"minute(now())", "extract(minute from now())");
}
#[test]
fn test_minute1(){
    test_expression(NSQL, "extract(minute from now())", "minute(now())");
}

#[test]
fn test_pgsql_minute1(){
    test_expression(PostgreSQL,"extract(minute from now())", "extract(minute from now())");
}


#[test]
fn test_month_add(){
    test_expression(NSQL, "month_add(now(),3)", "month_add(now(), 3)");
}

#[test]
fn test_pgsql_month_add(){
    test_expression(PostgreSQL,"month_add(now(),3)", "now() + interval '3' month");
}


#[test]
fn test_month_sub(){
    test_expression(NSQL, "month_sub(now(),3)", "month_sub(now(), 3)");
}

#[test]
fn test_pgsql_month_sub(){
    test_expression(PostgreSQL,"month_sub(now(),3)", "now() - interval '3' month");
}


#[test]
fn test_month(){
    test_expression(NSQL, "month(now())", "month(now())");
}

#[test]
fn test_pgsql_month(){
    test_expression(PostgreSQL,"month(now())", "extract(month from now())");
}


#[test]
fn test_month1(){
    test_expression(NSQL, "extract(month from now())", "month(now())");
}

#[test]
fn test_pgsql_month1(){
    test_expression(PostgreSQL,"extract(month from now())", "extract(month from now())");
}

#[test]
fn test_second_add(){
    test_expression(NSQL, "second_add(now(),3)", "second_add(now(), 3)");
}

#[test]
fn test_pgsql_second_add(){
    test_expression(PostgreSQL,"second_add(now(),3)", "now() + interval '3' second");
}

#[test]
fn test_second_sub(){
    test_expression(NSQL, "second_sub(now(),3)", "second_sub(now(), 3)");
}

#[test]
fn test_pgsql_second_sub(){
    test_expression(PostgreSQL,"second_sub(now(),3)", "now() - interval '3' second");
}

#[test]
fn test_second(){
    test_expression(NSQL, "second(now())", "second(now())");
}

#[test]
fn test_pgsql_second(){
    test_expression(PostgreSQL,"second(now())", "extract(second from now())");
}


#[test]
fn test_second1(){
    test_expression(NSQL, "extract(second from now())", "second(now())");
}


#[test]
fn test_pgsql_second1(){
    test_expression(PostgreSQL,"extract(second from now())", "extract(second from now())");
}

#[test]
fn test_year_add(){
    test_expression(NSQL,"year_add(now(),3)", "year_add(now(), 3)");
}

#[test]
fn test_pgsql_year_add(){
    test_expression(PostgreSQL, "year_add(now(),3)", "now() + interval '3' year");
}

#[test]
fn test_year_sub(){
    test_expression(NSQL, "year_sub(now(),3)", "year_sub(now(), 3)");
}

#[test]
fn test_pgsql_year_sub(){
    test_expression(PostgreSQL, "year_sub(now(),3)", "now() - interval '3' year");
}


#[test]
fn test_year(){
    test_expression(NSQL, "year(now())", "year(now())");
}

#[test]
fn test_pgsql_year(){
    test_expression(PostgreSQL, "year(now())", "extract(year from now())");
}


#[test]
fn test_year1(){
    test_expression(NSQL, "extract(year from abc)", "year(abc)");
}

#[test]
fn test_pgsql_year1(){
    test_expression(PostgreSQL, "extract(year from abc)", "extract(year from abc)");
}

#[test]
fn test_year_err(){
    let expr = ExpressionParser::new().parse(Lexer::new("extract(year form abc)").tokenizer());
    assert_eq!(true, expr.is_err());
}