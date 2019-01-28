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
fn test_equal(){
    test_predicate("a = c", "a = c");
}

#[test]
fn test_equal1(){
    test_predicate("a = 'abc'", "a = 'abc'");
}

#[test]
fn test_not_equal(){
    test_predicate("a != 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal1(){
    test_predicate("a ~= 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal2(){
    test_predicate("a ^= 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal3(){
    test_predicate("a <> 'abc'", "a <> 'abc'");
}


#[test]
fn test_less(){
    test_predicate("a <3", "a < 3");
}

#[test]
fn test_less1(){
    test_predicate("a <-3", "a < -3");
}


#[test]
fn test_less2(){
    test_predicate("a < 36.266", "a < 36.266");
}


#[test]
fn test_greater(){
    test_predicate("a>3", "a > 3");
}

#[test]
fn test_greater_variable(){
    test_predicate("a>:abc", "a > :abc");
}

#[test]
fn test_greater1(){
    test_predicate("我的字段>3", "我的字段 > 3");
}

