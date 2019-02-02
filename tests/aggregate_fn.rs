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
#[case("avg(a)", NSQL, "avg(a)")]
#[case("avg(distinct a)", NSQL, "avg(distinct a)")]
#[case("count(a)", NSQL, "count(a)")]
#[case("count(unique a)", NSQL, "count(unique a)")]
#[case("sum(all a)", NSQL, "sum(all a)")]
#[case("sum(a)", NSQL, "sum(a)")]
#[case("stddev(a)", NSQL, "stddev(a)")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right );
}

#[test]
fn test_avg2(){
    test_predicate(NSQL,"avg(a) >9", "avg(a) > 9");
}




