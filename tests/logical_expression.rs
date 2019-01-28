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
fn test_and(){
    test_predicate("a = 3 And c is null", "a = 3 and c is null");
}

#[test]
fn test_or(){
    test_predicate("a = 3 or c < 5", "a = 3 or c < 5");
}

#[test]
fn test_priority(){
    test_predicate("a =3   and (c < 5 or c= 7)", "a = 3 and (c < 5 or c = 7)");
}


#[test]
fn test_superfluous_brackets(){
    test_predicate("(a =3 )  and ((c < 5 or c= 7))", "a = 3 and (c < 5 or c = 7)");
}


#[test]
fn test_superfluous_brackets1(){
    test_predicate("(a =3 )  and (((c < 5 or c= 7)))", "a = 3 and (c < 5 or c = 7)");
}