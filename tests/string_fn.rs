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
#[case("concat(a, b ,c)", NSQL,  "concat(a, b, c)")]
#[case("concat('a', 'b' ,'c')", NSQL,  "concat('a', 'b', 'c')")]
#[case("left(a, 3)", NSQL, "left(a, 3)")]
#[case("UPPER(a)", NSQL, "upper(a)")]
#[case("Upper('abc')", NSQL, "upper('abc')")]
#[case("lower(a)", NSQL, "lower(a)")]
#[case("lower('abc')", NSQL,  "lower('abc')")]
#[case("LENGTH(a)", NSQL, "length(a)")]
#[case("Length('abc')", NSQL, "length('abc')")]
#[case("pad_left('abc', 3)", NSQL, "pad_left('abc', 3)")]
#[case("pad_left('abc', 3, 'a')", NSQL, "pad_left('abc', 3, 'a')")]
#[case("lpad('abc', 3, 'a')", NSQL, "pad_left('abc', 3, 'a')")]
#[case("pad_right('abc', 3)", NSQL,  "pad_right('abc', 3)")]
#[case("pad_right('abc', 3, 'a')", NSQL, "pad_right('abc', 3, 'a')")]
#[case("rpad('abc', 3, 'a')", NSQL, "pad_right('abc', 3, 'a')")]
#[case("replace('abc', '88')", NSQL, "replace('abc', '88')")]
#[case("reverse('abc')", NSQL, "reverse('abc')")]
#[case("substr('abc', 2)", NSQL, "substr('abc', 2)")]
#[case("substr('abc', 1,1)", NSQL, "substr('abc', 1, 1)")]
#[case("trim_end('abc ')", NSQL, "trim_end('abc ')")]
#[case("trim_end('abc ', 'd')", NSQL, "trim_end('abc ', 'd')")]
#[case("rtrim('abc', 'a')", NSQL, "trim_end('abc', 'a')")]
#[case("trim(trailing  'a' from 'abc')", NSQL, "trim_end('abc', 'a')")]
#[case("trim(trailing from 'abc ')", NSQL, "trim_end('abc ')")]
#[case("trim('abc ')", NSQL,  "trim('abc ')")]
#[case("trim('abc ', 'a')", NSQL, "trim('abc ', 'a')")]
#[case("btrim('abc ')", NSQL,  "trim('abc ')")]
#[case("btrim('abc ', 'a')", NSQL, "trim('abc ', 'a')" )]
#[case("trim_start(' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim_start(' abc ', 'a')", NSQL,  "trim_start(' abc ', 'a')")]
#[case("ltrim(' abc ', 'a')", NSQL, "trim_start(' abc ', 'a')")]
#[case("ltrim(' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim(leading from ' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim(leading 'a' from ' abc ')", NSQL, "trim_start(' abc ', 'a')")]
fn test1(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}
