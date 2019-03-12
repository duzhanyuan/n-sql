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
#[case("cast(a as text)",NSQL,"a::text")]
#[case("cast(1 as text)",NSQL,"1::text")]
#[case("cast('1' as int)",NSQL,"'1'::int")]
#[case("a::text",NSQL,"a::text")]

#[case("abc:: text", NSQL, "abc::text")]
#[case("abc:: text", PostgreSQL, "abc::text")]
//#[case("abc:: text", Oracle, "to_char(abc)")]
//#[case("abc:: text", MySQL, "convert(abc, char)")]
//#[case("abc:: text", SqlServer, "convert(varchar(8000), abc)")]
//#[case("abc:: text", SQLite, "cast(abc as text)")]


#[case("abc::int", NSQL, "abc::int")]
#[case("abc::int", PostgreSQL, "abc::int")]
//#[case("abc::int", Oracle, "cast(abc as int)")]
//#[case("abc::int", MySQL, "convert(abc, int)")]
//#[case("abc::int", SqlServer, "cast(abc as int)")]
//#[case("abc::int", SQLite, "cast(abc as int)")]

#[case("abc::float", NSQL, "abc::float")]
#[case("abc::float", PostgreSQL, "abc::float")]
//#[case("abc::float", Oracle, "cast(abc as float)")]
//#[case("abc::float", MySQL, "convert(abc, decimal(65, 38))")]
//#[case("abc::float", SqlServer, "cast(abc as float)")]
//#[case("abc::float", SQLite, "cast(abc as float)")]

#[case("abc::numeric", NSQL, "abc::numeric")]
#[case("abc::numeric", PostgreSQL, "abc::numeric")]
//#[case("abc::numeric", Oracle, "cast(abc as numeric)")]
//#[case("abc::numeric", MySQL, "convert(abc, decimal(65, 38))")]
//#[case("abc::numeric", SqlServer, "convert(numeric(38, 19), abc)")]
//#[case("abc::numeric", SQLite, "cast(abc as numeric)")]

#[case("abc::date", NSQL, "abc::date")]
#[case("abc::date", PostgreSQL, "abc::date")]
//#[case("abc::date", Oracle, "to_date(abc, 'yyyy-mm-dd')")]
//#[case("abc::date", MySQL, "convert(abc, date)")]
//#[case("abc::date", SqlServer, "convert(date, abc)")]
//#[case("abc::date", SQLite, "date(abc)")]

#[case("abc::datetime", NSQL, "abc::datetime")]
#[case("abc::datetime", PostgreSQL, "abc::datetime")]
//#[case("abc::datetime", Oracle, "to_timestamp(abc, 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("abc::datetime", MySQL, "convert(abc, datetime)")]
//#[case("abc::datetime", SqlServer, "convert(datetime, abc)")]
//#[case("abc::datetime", SQLite, "datetime(abc)")]

#[case("abc::timestamp", NSQL, "abc::timestamp")]
#[case("abc::timestamp", PostgreSQL, "abc::timestamp")]
//#[case("abc::timestamp", Oracle, "to_timestamp(abc, 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("abc::timestamp", MySQL, "convert(abc, datetime)")]
//#[case("abc::timestamp", SqlServer, "convert(datetime, abc)")]
//#[case("abc::timestamp", SQLite, "datetime(abc)")]

#[case("abc::time", NSQL, "abc::time")]
#[case("abc::time", PostgreSQL, "abc::time")]
//#[case("abc::time", Oracle, "")]
//#[case("abc::time", MySQL, "convert(abc, time)")]
//#[case("abc::time", SqlServer, "convert(time, abc)")]
//#[case("abc::time", SQLite, "time(abc)")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}