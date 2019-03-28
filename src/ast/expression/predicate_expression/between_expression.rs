// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct BetweenExpression {
    pub expr: Box<Expression>,
    pub begin: Box<Expression>,
    pub end: Box<Expression>
}

impl BetweenExpression {
    pub fn new(expr: Box<Expression>, begin: Box<Expression>, end: Box<Expression>) -> BetweenExpression {
        BetweenExpression{expr, begin, end}
    }
}