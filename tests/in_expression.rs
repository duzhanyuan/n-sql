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
#[case("a in ('x',     'y',      'z')" , NSQL, "a in ('x', 'y', 'z')")]
#[case("a in (1,     3,      5)" , NSQL,"a in (1, 3, 5)")]
#[case("a in ('2')" , NSQL,"a in ('2')")]
#[case("a in (select name from student)" , NSQL,"a in (select name from student)")]
#[case("a not in ('x',     'y',      'z')" , NSQL, "a not in ('x', 'y', 'z')")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_predicate(database_type, left, right)
}
