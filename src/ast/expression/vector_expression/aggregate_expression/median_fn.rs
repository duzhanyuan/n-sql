// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::AggregateType;
use ast::{Expression, PredicateExpression};

#[derive(Clone, Debug)]
pub struct MedianFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MedianFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> MedianFn {
        MedianFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MedianIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MedianIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> MedianIfFn {
        MedianIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}
