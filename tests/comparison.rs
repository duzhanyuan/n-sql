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
#[case("a = c", NSQL, "a = c")]
#[case("a = 'abc'" , NSQL, "a = 'abc'")]
#[case("a != 'abc'" , NSQL, "a <> 'abc'")]
#[case( "a ~= 'abc'", NSQL, "a <> 'abc'")]
#[case("a ^= 'abc'" , NSQL, "a <> 'abc'")]
#[case("a <> 'abc'" , NSQL, "a <> 'abc'")]
#[case("a <3" , NSQL,"a < 3")]
#[case( "a <-3", NSQL,"a < -3")]
#[case("a < 36.266" , NSQL,"a < 36.266")]
#[case("a>3" , NSQL,"a > 3")]
#[case("a>:abc" , NSQL,"a > :abc")]
#[case( "我的字段>3", NSQL,"我的字段 > 3")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_predicate(database_type, left, right)
}

