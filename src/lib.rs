// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(const_fn)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate core;
extern crate cfg_if;
#[macro_use] extern crate failure;

use cfg_if::cfg_if;
mod ast;
mod optimizer;
mod grammar;
mod generator;
mod version;
mod lexer;
mod catalog;
pub mod parser;

pub use ast::*;
pub use generator::*;
pub use optimizer::Optimizer;
pub use grammar::{
    PredicateEntryParser as PredicateParser,
    ExpressionEntryParser as ExpressionParser,
    StatementEntryParser as StatementParser};
pub use lexer::Lexer;


cfg_if!{
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(all(target_arch="wasm32", feature="wee_alloc"))] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        extern crate wasm_bindgen;

        mod wasm_utils;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            fn alert(s: &str);
        }

        #[wasm_bindgen]
        pub fn translate(sql: &str) -> String {
            let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
            statement.to_sql().unwrap()
        }

        #[wasm_bindgen]
        pub fn translate_pgsql(sql: &str) -> String {
            let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
            statement.to_pgsql().unwrap()
        }

        #[wasm_bindgen]
        pub fn translate_oracle(sql: &str) -> String {
            let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
            statement.to_oracle().unwrap()
        }

        #[wasm_bindgen]
        pub fn translate_mysql(sql: &str) -> String {
            let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
            statement.to_mysql().unwrap()
        }
    }
}