// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(dead_code)]

extern crate n_sql;

use self::n_sql::{
    ExpressionParser, StatementParser, PredicateParser, Lexer,
    Generator, PgsqlGenerator, OracleGenerator, MySQLGenerator,
};


pub fn test_expression(left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer())
    .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}

pub fn test_statement(left: &str, right:&str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}
pub fn test_predicate(left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}


pub fn test_pgsql_expression(left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_pgsql().unwrap());
}

pub fn test_pgsql_statement(left: &str, right:&str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_pgsql().unwrap());
}
pub fn test_pgsql_predicate(left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_pgsql().unwrap());
}



pub fn test_oracle_expression(left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_oracle().unwrap());
}

pub fn test_oracle_statement(left: &str, right:&str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_oracle().unwrap());
}
pub fn test_oracle_predicate(left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_oracle().unwrap());
}

pub fn test_mysql_expression(left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_mysql().unwrap());
}

pub fn test_mysql_statement(left: &str, right:&str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_mysql().unwrap());
}
pub fn test_mysql_predicate(left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_mysql().unwrap());
}
