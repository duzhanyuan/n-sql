// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Visitor;
use ast::*;
use optimizer::Optimizer;
use std::fmt::{Error, Result, Write};
use std::result;

type Formatter = String;

pub trait OracleGenerator<T> {
    fn to_oracle(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_pagination_statement(
        &self,
        pagination_statement: &Box<PaginationStatement>,
        f: &mut Formatter,
    ) -> Result {
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
                    }
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
                    }
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


    fn visit_select_statement(&self, statement: &SelectStatement, f: &mut Formatter) -> Result {
        f.write_str("select")?;
        if let Some(ref t) = statement.select_type {
            f.write_char(' ')?;
            use SelectType::*;
            match t {
                All => f.write_str("all")?,
                Distinct => f.write_str("distinct")?,
                Unique => f.write_str("unique")?,
            }
        }

        f.write_char(' ')?;
        self.visit_select_elements(&statement.elements, f)?;

        if let Some(ref tables) = statement.tables {
            f.write_char(' ')?;
            f.write_str("from")?;
            f.write_char(' ')?;
            self.visit_tables(tables, f)?;

            if let Some(ref w) = statement.predicate {
                f.write_char(' ')?;
                f.write_str("where")?;
                f.write_char(' ')?;
                self.visit_predicate(w, f)?;
            }

            if let Some(ref g) = statement.group {
                f.write_char(' ')?;
                f.write_str("group")?;
                f.write_char(' ')?;
                f.write_str("by")?;
                f.write_char(' ')?;
                self.visit_expressions(&g.elements, f)?;

                if let Some(ref h) = g.having {
                    f.write_char(' ')?;
                    f.write_str("having")?;
                    f.write_char(' ')?;
                    self.visit_predicate(h, f)?;
                }
            }

            if let Some(ref s) = statement.sorts {
                f.write_char(' ')?;
                f.write_str("order")?;
                f.write_char(' ')?;
                f.write_str("by")?;
                f.write_char(' ')?;
                self.visit_sorting_elements(s, f)?;
            }
        } else {
            f.write_char(' ')?;
            f.write_str("from")?;
            f.write_char(' ')?;
            f.write_str("dual")?;
        }
        Ok(())
    }
    fn visit_now_fn(&self, f: &mut Formatter) -> Result {
        f.write_str("systimestamp")
    }
    fn visit_cast_fn(&self, function: &Box<CastFn>, f: &mut Formatter) -> Result {
        match function.data_type.data_type.to_lowercase().as_str() {
            "text" => {
                f.write_str("to_char")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_char(')')
            }
            "date" => {
                f.write_str("to_date")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_str(", 'yyyy-mm-dd'")?;
                f.write_char(')')
            }
            "timestamp" | "datetime" => {
                f.write_str("to_timestamp")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_str(", 'yyyy-mm-dd hh24:mi:ss'")?;
                f.write_char(')')
            }
            _ => {
                f.write_str("cast")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_str(" as ")?;
                f.write_str(&function.data_type.data_type)?;
                f.write_char(')')
            }
        }
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
