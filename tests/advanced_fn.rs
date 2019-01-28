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
fn test_nvl(){
    test_expression("nvl(a, 'no value')", "nvl(a, 'no value')");
}

#[test]
fn test_mysql_nvl(){
    test_mysql_expression("nvl(a, 'no value')", "ifnull(a, 'no value')");
}

#[test]
fn test_pgsql_nvl(){
    test_pgsql_expression("nvl(a, 'no value')", "coalesce(a, 'no value')");
}


#[test]
fn test_nvl1(){
    test_expression("NVL(a, 'no value')", "nvl(a, 'no value')");
}

#[test]
fn test_nvl2(){
    test_expression("NVL(a,      'no value')", "nvl(a, 'no value')");
}


#[test]
fn test_nvl3(){
    test_expression("NVL(a,      123)", "nvl(a, 123)");
}


#[test]
fn test_nvl4(){
    test_expression("NVL(a,      -123)", "nvl(a, -123)");
}

#[test]
fn test_coalesce(){
    test_expression("COALESCE(a,b,c)", "coalesce(a, b, c)");
}