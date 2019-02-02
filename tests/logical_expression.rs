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
#[case("a = 3 And c is null", NSQL, "a = 3 and c is null")]
#[case("a = 3 or c < 5", NSQL, "a = 3 or c < 5")]
#[case("a =3   and (c < 5 or c= 7)", NSQL, "a = 3 and (c < 5 or c = 7)")]
#[case("(a =3 )  and ((c < 5 or c= 7))", NSQL, "a = 3 and (c < 5 or c = 7)")]
#[case("(a =3 )  and (((c < 5 or c= 7)))", NSQL, "a = 3 and (c < 5 or c = 7)")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_predicate(database_type, left, right)
}
