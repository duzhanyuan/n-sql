// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::super::{Expression, NumericValue, StringValue};
use crate::ast::SetStatement;

#[derive(Clone, Debug)]
pub struct InExpression {
    pub left: Box<Expression>,
    pub values: InElements,
}

impl InExpression {
    pub fn new(left: Box<Expression>, values: InElements) -> InExpression {
        InExpression { left, values }
    }
}

#[derive(Clone, Debug)]
pub enum InElements {
    Values(Vec<Box<Expression>>),
    Set(Box<SetStatement>),
}
