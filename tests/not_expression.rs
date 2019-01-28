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
fn not_equal_comparison(){
    test_predicate("nOt(a = 'abc')", "not(a = 'abc')");
}

#[test]
fn not_logical(){
    test_predicate("nOt(a = 'abc' and age <3)", "not(a = 'abc' and age < 3)");
}