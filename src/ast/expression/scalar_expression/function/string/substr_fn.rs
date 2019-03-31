// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::ast::{Expression as UnBoxExpression, LeftFn};

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct SubstrFn {
    pub text: Expression,
    pub start: Expression,
    pub length: Option<Expression>,
}

impl SubstrFn {
    pub fn new(text: Expression, start: Expression, length: Option<Expression>) -> SubstrFn {
        SubstrFn {
            text,
            start,
            length,
        }
    }
}

impl From<&LeftFn> for SubstrFn {
    fn from(v: &LeftFn) -> Self {
        SubstrFn::new(
            v.text.clone(),
            UnBoxExpression::from(1).into(),
            v.length.clone().into())
    }
}
