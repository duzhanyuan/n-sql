// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::*;
use super::Visitor;
use std::result;
use std::fmt::{Write, Error, Result};
use optimizer::Optimizer;

type Formatter = String;

pub trait OracleGenerator<T>{
    fn to_oracle(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator{
    fn visit_pagination_statement(&self, pagination_statement: &Box<PaginationStatement>, f: &mut Formatter) -> Result {
        self.visit_set_statement(&pagination_statement.set, f)?;
        if let Some(ref skip) = pagination_statement.skip {
            f.write_char(' ')?;
            f.write_str("offset")?;
            f.write_char(' ')?;

            self.visit_expression(skip, f)?;
            if let Some(skip) = skip.as_ref().constant_numeric() {
                match skip {
                    NumericValue::Integer(integer) => {
                        if integer == 1 {
                            f.write_str(" row")?;
                        } else {
                            f.write_str(" rows")?;
                        }
                    },
                    NumericValue::Float(float) => {
                        if float == 1.0 {
                            f.write_str(" row")?;
                        } else {
                            f.write_str(" rows")?;
                        }
                    }
                }
            } else {
                f.write_str(" rows")?;
            }
        }

        if let Some(ref limit) = pagination_statement.limit {
            f.write_char(' ')?;
            f.write_str("fetch")?;
            f.write_char(' ')?;
            f.write_str("first")?;
            f.write_char(' ')?;
            self.visit_expression(limit, f)?;

            if let Some(limit) = limit.as_ref().constant_numeric() {
                match limit {
                    NumericValue::Integer(integer) => {
                        if integer == 1 {
                            f.write_str(" row ")?;
                        } else {
                            f.write_str(" rows ")?;
                        }
                    },
                    NumericValue::Float(float) => {
                        if float == 1.0 {
                            f.write_str(" row ")?;
                        } else {
                            f.write_str(" rows ")?;
                        }
                    }
                }
            } else {
                f.write_str(" rows ")?;
            }

            f.write_str("only")?;
        }
        Ok(())
    }
}

impl OracleGenerator<Expression> for Expression {
    fn to_oracle(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}


impl OracleGenerator<PredicateExpression> for PredicateExpression {
    fn to_oracle(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl OracleGenerator<Statement> for Statement {
    fn to_oracle(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}