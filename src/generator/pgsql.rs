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

//noinspection SpellCheckingInspection
pub trait PgsqlGenerator<T> {
    fn to_pgsql(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_median_fn(&self, function: &MedianFn, f: &mut Formatter) -> Result {
        self.visit_percentile(&PercentileFn::from(function), f)
    }


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
        }

        if let Some(ref limit) = pagination_statement.limit {
            f.write_char(' ')?;
            f.write_str("limit")?;
            f.write_char(' ')?;
            self.visit_expression(limit, f)?;
        }
        Ok(())
    }

    fn visit_log10_fn(&self, function: &Log10Fn, f: &mut Formatter) -> Result {
        f.write_str("log")?;
        f.write_char('(')?;
        f.write_str("10, ")?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }

    fn visit_log_fn(&self, function: &LogFn, f: &mut Formatter) -> Result {
        f.write_str("log")?;
        f.write_char('(')?;
        if let Some(ref t) = function.base {
            self.visit_expression(t, f)?;
        } else {
            let exp = ExpFn::new(Expression::from(1).into());
            self.visit_exp_fn(&exp, f)?;
        }
        f.write_str(", ")?;
        self.visit_expression(&function.number, f)?;
        f.write_char(')')
    }
    fn visit_extract_fn(&self, function: &ExtractFn, f: &mut Formatter) -> Result {
        f.write_str("extract")?;
        f.write_char('(')?;

        self.visit_datetime_type(&function.extract_type, f)?;
        f.write_char(' ')?;
        f.write_str("from")?;
        f.write_char(' ')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_year_add_fn(&self, function: &YearAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("year")
    }
    fn visit_year_sub_fn(&self, function: &YearSubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("year")
    }
    fn visit_month_add_fn(&self, function: &MonthAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("month")
    }
    fn visit_month_sub_fn(&self, function: &MonthSubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("month")
    }
    fn visit_day_add_fn(&self, function: &DayAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("day")
    }

    fn visit_day_sub_fn(&self, function: &DaySubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("day")
    }

    fn visit_hour_add_fn(&self, function: &HourAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("hour")
    }
    fn visit_hour_sub_fn(&self, function: &HourSubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("hour")
    }
    fn visit_minute_add_fn(&self, function: &MinuteAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("minute")
    }
    fn visit_minute_sub_fn(&self, function: &MinuteSubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("minute")
    }
    fn visit_second_add_fn(&self, function: &SecondAddFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" + ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("second")
    }
    fn visit_second_sub_fn(&self, function: &SecondSubFn, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str(" - ")?;
        f.write_str("interval")?;
        f.write_char(' ')?;
        f.write_char('\'')?;
        self.visit_expression(&function.offset.optimize(), f)?;
        f.write_char('\'')?;
        f.write_char(' ')?;
        f.write_str("second")
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

//noinspection SpellCheckingInspection
impl PgsqlGenerator<Expression> for Expression {
    fn to_pgsql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}

//noinspection SpellCheckingInspection
impl PgsqlGenerator<PredicateExpression> for PredicateExpression {
    fn to_pgsql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

//noinspection SpellCheckingInspection
impl PgsqlGenerator<Statement> for Statement {
    fn to_pgsql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}
