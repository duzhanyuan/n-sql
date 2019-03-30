// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Visitor;
use crate::ast::*;
use crate::optimizer::Optimizer;
use std::fmt::{Error, Result, Write};
use std::result;

type Formatter = String;

pub trait SqlServerGenerator<T> {
    fn to_sql_server(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_median_fn(&self, function: &MedianFn, f: &mut Formatter) -> Result {
        self.visit_percentile(&PercentileFn::from(function), f)
    }
    fn visit_percentile(&self, function: &PercentileFn, f: &mut Formatter) -> Result {
        match function.r#type {
            PercentileType::Cont => f.write_str("percentile_cont")?,
            PercentileType::Disc => f.write_str("percentile_disc")?,
        };
        f.write_char('(')?;
        self.visit_expression(&function.p, f)?;
        f.write_char(')')?;
        f.write_str(" within group (order by ")?;
        self.visit_expression(&function.expr, f)?;

        if let Some(t) = function.order.as_ref() {
            match t {
                SortingDirection::Ascending => f.write_str(" asc")?,
                SortingDirection::Descending => f.write_str(" desc")?,
            }
        }
        f.write_char(')')?;
        f.write_str(" over (partition by 0)")
    }
    fn visit_pagination_statement(
        &self,
        pagination_statement: &Box<PaginationStatement>,
        f: &mut Formatter,
    ) -> Result {

        self.visit_set_statement(&pagination_statement.set, f)?;

        let has_order = ||match pagination_statement.set.as_ref() {
            SetStatement::Select(t) if t.sorts.is_some() => true,
            _ =>  false
        };

        let mut has_add_order = false;

        if let Some(ref skip) = pagination_statement.skip {
            if !has_order(){
                f.write_str(" order by 1")?;
                has_add_order = true;
            }
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
            if !has_add_order{
                if !has_order() {
                    f.write_str(" order by 1")?;
                }

                f.write_str(" offset 0 row")?;
            }

            f.write_char(' ')?;
            f.write_str("fetch")?;
            f.write_char(' ')?;
            f.write_str("next")?;
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
    fn visit_now_fn(&self, f: &mut Formatter) -> Result {
        f.write_str("systimestamp")
    }
    fn visit_ceil_fn(&self, function: &CeilFn, f: &mut Formatter) -> Result {
        f.write_str("ceiling")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }

    fn visit_log_fn(&self, function: &LogFn, f: &mut Formatter) -> Result {
        f.write_str("log")?;
        f.write_char('(')?;
        self.visit_expression(&function.number, f)?;
        if let Some(ref t) = function.base {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }

    fn visit_nvl_fn(&self, function: &Box<NvlFn>, f: &mut Formatter) -> Result {
        f.write_str("coalesce")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.default, f)?;
        f.write_char(')')
    }

    fn visit_cast_fn(&self, function: &Box<CastFn>, f: &mut Formatter) -> Result {
        let lower_tp = function.data_type.data_type.to_lowercase();
        let tp = lower_tp.as_str();
        match tp {
            "int" | "float" => {
                f.write_str("cast")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_str(" as ")?;
                f.write_str(&function.data_type.data_type)?;
                f.write_char(')')
            }
            other => {
                f.write_str("convert")?;
                f.write_char('(')?;
                f.write_str(match other {
                    "text" => "varchar(8000)",
                    "timestamp" => "datetime",
                    "numeric" | "float" => "numeric(38, 19)",
                    _ => &function.data_type.data_type,
                })?;
                f.write_str(", ")?;
                self.visit_expression(&function.expr, f)?;
                f.write_char(')')
            }
        }
    }
}

impl SqlServerGenerator<Expression> for Expression {
    fn to_sql_server(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}

impl SqlServerGenerator<PredicateExpression> for PredicateExpression {
    fn to_sql_server(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl SqlServerGenerator<Statement> for Statement {
    fn to_sql_server(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}
