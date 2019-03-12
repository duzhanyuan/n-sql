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
#[case("abs(-6)", NSQL, "abs(-6)")]
#[case("abs(9)", NSQL, "abs(9)")]

#[case("ceil(-6.8)", NSQL, "ceil(-6.8)")]
#[case("ceil(-6.8)", PostgreSQL, "ceil(-6.8)")]
#[case("ceil(-6.8)", Oracle, "ceil(-6.8)")]
#[case("ceil(-6.8)", MySQL, "ceil(-6.8)")]
//#[case("ceil(-6.8)", SqlServer, "ceiling(-6.8)")]
//#[case("ceil(-6.8)", SQLite, "ceil(-6.8)")]



#[case("cos(6.8)", NSQL, "cos(6.8)")]
#[case("cos(6.8)", PostgreSQL, "cos(6.8)")]
#[case("cos(6.8)", Oracle, "cos(6.8)")]
#[case("cos(6.8)", MySQL, "cos(6.8)")]
//#[case("cos(6.8)", SQLite, "cos(6.8)")]




#[case("dense_rank(a)", NSQL, "dense_rank(a)")]
#[case("dense_rank(a  , asc)", NSQL, "dense_rank(a, asc)")]
#[case("dense_rank(a  , desc)", NSQL, "dense_rank(a, desc)")]
#[case("floor(-6.8)", NSQL, "floor(-6.8)")]
#[case("log10(6.8)", NSQL, "log10(6.8)")]
#[case("log(6.8)", NSQL, "log(6.8)")]
#[case("log(6.8, 2)", NSQL, "log(6.8, 2)")]
#[case("pow(6.8, 2)", NSQL, "pow(6.8, 2)")]
#[case("power(6.8, 2)", NSQL, "pow(6.8, 2)")]
#[case("rank(a)", NSQL, "rank(a)")]
#[case("rank(a  , asc)", NSQL, "rank(a, asc)")]
#[case("rank(a  , desc)", NSQL, "rank(a, desc)")]
#[case("round(6.855, 2)", NSQL, "round(6.855, 2)")]
#[case("sign(-6.855)", NSQL, "sign(-6.855)")]
#[case("sin(6.855)", NSQL, "sin(6.855)")]
#[case("sqrt(6.855)", NSQL, "sqrt(6.855)")]
#[case("tan(6.855)", NSQL, "tan(6.855)")]
fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}
