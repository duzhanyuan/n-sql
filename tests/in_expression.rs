// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod common;
use common::*;

#[test]
fn test_in(){
    test_predicate("a in ('x',     'y',      'z')", "a in ('x', 'y', 'z')");
}

#[test]
fn test_in1(){
    test_predicate("a in (1,     3,      5)", "a in (1, 3, 5)");
}

//#[test]
//fn test_in_empty(){
//    let expr = PredicateParser::new()
//        .parse("a in ()");
//
//    assert_eq!(true, expr.is_err());
//}

#[test]
fn test_in_one_element() {
    test_predicate("a in ('2')", "a in ('2')");
}

#[test]
fn test_in_subquery(){
    test_predicate("a in (select name from student)",
                   "a in (select name from student)");
}

#[test]
fn test_not_in(){
    test_predicate("a not in ('x',     'y',      'z')", "a not in ('x', 'y', 'z')");
}