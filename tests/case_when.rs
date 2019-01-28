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
fn test_simple(){
    test_expression("case   a when 1 then '甲' when 2 then '乙' End", "case a when 1 then '甲' when 2 then '乙' end");
}

#[test]
fn test_simple1(){
    test_expression("case a when 1 then '甲' when 2 then '乙' ELSE '其他' End", "case a when 1 then '甲' when 2 then '乙' else '其他' end");
}


#[test]
fn test_search(){
    test_expression("CASE   when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' else '未知' End",
                    "case when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' else '未知' end");
}

#[test]
fn test_search1(){
    test_expression("CASE   when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' End",
                    "case when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' end");
}