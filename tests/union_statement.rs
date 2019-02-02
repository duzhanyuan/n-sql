// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate n_sql;

use n_sql::StatementParser;
use n_sql::Lexer;

#[macro_use]
mod common;


test_init!();

#[test]
fn test_union(){
    test_statement(NSQL, "select name from student where gender ='男' union select name from student where gender ='女'",
                   "select name from student where gender = '男' union select name from student where gender = '女'");
}

#[test]
fn test_union1(){
    test_statement(NSQL, "select name from student where gender ='男' union select name from student where gender ='女' skip 1 limit 2",
                   "select name from student where gender = '男' union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union2(){
    let statement = StatementParser::new()
        .parse(Lexer::new("select name from student where gender ='男' skip 2 union select name from student where gender ='女' skip 1 limit 2").tokenizer());
    assert_eq!(true, statement.is_err());
}

#[test]
fn test_union3(){
    test_statement(NSQL, "(select name from student where gender ='男' skip 2) union select name from student where gender ='女' skip 1 limit 2",
                   "(select name from student where gender = '男' skip 2) union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union_all(){
    test_statement(NSQL, "select name from student where gender ='男' union all select name from student where gender ='女'",
                   "select name from student where gender = '男' union all select name from student where gender = '女'");
}

