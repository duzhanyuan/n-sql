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
#[case("nvl(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("nvl(a, 'no value')", MySQL, "ifnull(a, 'no value')")]
#[case("nvl(a, 'no value')", PostgreSQL, "coalesce(a, 'no value')")]
#[case("NVL(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("NVL(a,      123)", NSQL, "nvl(a, 123)")]
#[case("NVL(a,      -123)", NSQL, "nvl(a, -123)")]
fn test_nvl(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}


#[theory]
#[case("COALESCE(a,b,c)", NSQL, "coalesce(a, b, c)")]
fn test_coalesce(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}