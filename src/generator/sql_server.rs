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

pub trait SqlServerGenerator<T> {
    fn to_sql_server(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_percentile(&self, function: &PercentileFn, f: &mut Formatter) -> Result {
        match function.r#type {
            PercentileType::Cont => f.write_str("percentile_cont")?,
            PercentileType::Disc => f.write_str("percentile_disc")?
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


    fn visit_nvl_fn(&self, function: &Box<NvlFn>, f: &mut Formatter) -> Result {
        f.write_str("coalesce")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.default, f)?;
        f.write_char(')')
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
