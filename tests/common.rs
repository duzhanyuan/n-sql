// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(dead_code)]
#![allow(unused_macros)]

extern crate fluid;
extern crate n_sql;

pub use self::fluid::prelude::*;
pub use self::n_sql::{
    ExpressionParser, StatementParser, PredicateParser, Lexer,
    Generator, PgsqlGenerator, OracleGenerator, MySQLGenerator,
};

pub use self::n_sql::parser;

pub enum DatabaseType {
    NSQL,
    MySQL,
    Oracle,
    PostgreSQL,
    SQLite
}


macro_rules! test_init {
    () => {
        use common::*;
        #[allow(unused_imports)]
        use DatabaseType::*;
        #[test]
        fn init() {
        }
    };
}


pub fn test_expression(database_type: DatabaseType, left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer());
//    expr.should().not().be_an_error();
    let expr = expr.unwrap();
    let result = match database_type {
        DatabaseType::NSQL => expr.to_sql(),
        DatabaseType::MySQL => expr.to_mysql(),
        DatabaseType::Oracle => expr.to_oracle(),
        DatabaseType::PostgreSQL => expr.to_pgsql(),
        _ => unimplemented!()
    }.unwrap();

    result.as_str().should().be_equal_to(right);
}

pub fn test_statement(database_type: DatabaseType, left: &str, right: &str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer());
//    expr.should().not().be_an_error();
    let expr = expr.unwrap();

    let result = match database_type {
        DatabaseType::NSQL => expr.to_sql(),
        DatabaseType::MySQL => expr.to_mysql(),
        DatabaseType::Oracle => expr.to_oracle(),
        DatabaseType::PostgreSQL => expr.to_pgsql(),
        _ => unimplemented!()
    }.unwrap();

    result.as_str().should().be_equal_to(right);
}
pub fn test_predicate(database_type: DatabaseType, left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer());
//    expr.should().not().be_an_error();
    let expr = expr.unwrap();
    let result = match database_type {
        DatabaseType::NSQL => expr.to_sql(),
        DatabaseType::MySQL => expr.to_mysql(),
        DatabaseType::Oracle => expr.to_oracle(),
        DatabaseType::PostgreSQL => expr.to_pgsql(),
        _ => unimplemented!()
    }.unwrap();

    result.as_str().should().be_equal_to(right);
}
