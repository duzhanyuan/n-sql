// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod common;
use common::*;

extern crate n_sql;

use n_sql::{ExpressionParser, Lexer};

#[test]
fn test_day(){
    test_expression("day(now())", "day(now())");
}

#[test]
fn test_pgsql_day(){
    test_pgsql_expression("day(now())", "extract(day from now())");
}

#[test]
fn test_day1(){
    test_expression("extract(day from now())", "day(now())");
}

#[test]
fn test_pgsql_day1(){
    test_pgsql_expression("extract(day from now())", "extract(day from now())");
}

#[test]
fn test_day_add(){
    test_expression("day_add(now(),3)", "day_add(now(), 3)");
}

#[test]
fn test_pgsql_day_add(){
    test_pgsql_expression("day_add(now(),3)", "now() + interval '3' day");
}

#[test]
fn test_day_add1(){
    test_expression("day_add(now(),3+9*7)", "day_add(now(), 3 + 9 * 7)");
}

#[test]
fn test_pgsql_day_add1(){
    test_pgsql_expression("day_add(now(),3+9*7)", "now() + interval '66' day");
}

#[test]
fn test_day_sub(){
    test_expression("day_sub(now(),3)", "day_sub(now(), 3)");
}

#[test]
fn test_pgsql_day_sub(){
    test_pgsql_expression("day_sub(now(),3)", "now() - interval '3' day");
}

#[test]
fn test_day_sub1(){
    test_expression("day_sub(create_date,3)", "day_sub(create_date, 3)");
}

#[test]
fn test_hour_add(){
    test_expression("HOUR_add(now(),3)", "hour_add(now(), 3)");
}


#[test]
fn test_pgsql_hour_add(){
    test_pgsql_expression("hour_add(now(),3)", "now() + interval '3' hour");
}


#[test]
fn test_hour_sub(){
    test_expression("HOUR_sub(now(),3)", "hour_sub(now(), 3)");
}

#[test]
fn test_pgsql_hour_sub(){
    test_pgsql_expression("HOUR_sub(now(),3)", "now() - interval '3' hour");
}


#[test]
fn test_hour(){
    test_expression("HOUR(now())", "hour(now())");
}
#[test]
fn test_pgsql_hour(){
    test_pgsql_expression("HOUR(now())", "extract(hour from now())");
}
#[test]
fn test_hour1(){
    test_expression("extract(hour from now())", "hour(now())");
}
#[test]
fn test_pgsql_hour1(){
    test_pgsql_expression("extract(hour from now())", "extract(hour from now())");
}

#[test]
fn test_minute_add(){
    test_expression("minute_add(now(),3)", "minute_add(now(), 3)");
}

#[test]
fn test_pgsql_minute_add(){
    test_pgsql_expression("minute_add(now(),3)", "now() + interval '3' minute");
}

#[test]
fn test_minute_sub(){
    test_expression("minute_sub(now(),3)", "minute_sub(now(), 3)");
}

#[test]
fn test_pgsql_minute_sub(){
    test_pgsql_expression("minute_sub(now(),3)", "now() - interval '3' minute");
}

#[test]
fn test_minute(){
    test_expression("minute(now())", "minute(now())");
}
#[test]
fn test_pgsql_minute(){
    test_pgsql_expression("minute(now())", "extract(minute from now())");
}
#[test]
fn test_minute1(){
    test_expression("extract(minute from now())", "minute(now())");
}

#[test]
fn test_pgsql_minute1(){
    test_pgsql_expression("extract(minute from now())", "extract(minute from now())");
}


#[test]
fn test_month_add(){
    test_expression("month_add(now(),3)", "month_add(now(), 3)");
}

#[test]
fn test_pgsql_month_add(){
    test_pgsql_expression("month_add(now(),3)", "now() + interval '3' month");
}


#[test]
fn test_month_sub(){
    test_expression("month_sub(now(),3)", "month_sub(now(), 3)");
}

#[test]
fn test_pgsql_month_sub(){
    test_pgsql_expression("month_sub(now(),3)", "now() - interval '3' month");
}


#[test]
fn test_month(){
    test_expression("month(now())", "month(now())");
}

#[test]
fn test_pgsql_month(){
    test_pgsql_expression("month(now())", "extract(month from now())");
}


#[test]
fn test_month1(){
    test_expression("extract(month from now())", "month(now())");
}

#[test]
fn test_pgsql_month1(){
    test_pgsql_expression("extract(month from now())", "extract(month from now())");
}

#[test]
fn test_second_add(){
    test_expression("second_add(now(),3)", "second_add(now(), 3)");
}

#[test]
fn test_pgsql_second_add(){
    test_pgsql_expression("second_add(now(),3)", "now() + interval '3' second");
}

#[test]
fn test_second_sub(){
    test_expression("second_sub(now(),3)", "second_sub(now(), 3)");
}

#[test]
fn test_pgsql_second_sub(){
    test_pgsql_expression("second_sub(now(),3)", "now() - interval '3' second");
}

#[test]
fn test_second(){
    test_expression("second(now())", "second(now())");
}

#[test]
fn test_pgsql_second(){
    test_pgsql_expression("second(now())", "extract(second from now())");
}


#[test]
fn test_second1(){
    test_expression("extract(second from now())", "second(now())");
}


#[test]
fn test_pgsql_second1(){
    test_pgsql_expression("extract(second from now())", "extract(second from now())");
}

#[test]
fn test_year_add(){
    test_expression("year_add(now(),3)", "year_add(now(), 3)");
}

#[test]
fn test_pgsql_year_add(){
    test_pgsql_expression("year_add(now(),3)", "now() + interval '3' year");
}

#[test]
fn test_year_sub(){
    test_expression("year_sub(now(),3)", "year_sub(now(), 3)");
}

#[test]
fn test_pgsql_year_sub(){
    test_pgsql_expression("year_sub(now(),3)", "now() - interval '3' year");
}


#[test]
fn test_year(){
    test_expression("year(now())", "year(now())");
}

#[test]
fn test_pgsql_year(){
    test_pgsql_expression("year(now())", "extract(year from now())");
}


#[test]
fn test_year1(){
    test_expression("extract(year from abc)", "year(abc)");
}

#[test]
fn test_pgsql_year1(){
    test_pgsql_expression("extract(year from abc)", "extract(year from abc)");
}

#[test]
fn test_year_err(){
    let expr = ExpressionParser::new().parse(Lexer::new("extract(year form abc)").tokenizer());
    assert_eq!(true, expr.is_err());

}