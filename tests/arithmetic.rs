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
fn test_add(){
    test_expression("a     +3","a + 3" );
}


#[test]
fn test_sub(){
    test_expression("a  - 3", "a - 3");
}

#[test]
fn test_mul(){
    test_expression("a* 3", "a * 3");
}


#[test]
fn test_div(){
    test_expression("a/ 3", "a / 3");
}

#[test]
fn test_priority(){
    test_expression("a/ 3 + 3", "a / 3 + 3");
}


#[test]
fn test_priority1(){
    test_expression("a/ (3 + 3)", "a / (3 + 3)");
}

#[test]
fn test_superfluous_brackets(){
    test_expression("(a)/ ((3 + 3))", "a / (3 + 3)");
}

#[test]
fn test_superfluous_brackets1(){
    test_expression("(a)/ ((m + n))", "a / (m + n)");
}