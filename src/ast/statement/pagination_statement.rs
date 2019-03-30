// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ast::Expression;
use crate::ast::SetStatement;

#[derive(Clone, Debug)]
pub struct PaginationStatement {
    pub set: Box<SetStatement>,
    pub skip: Option<Box<Expression>>,
    pub limit: Option<Box<Expression>>,
}

impl PaginationStatement {
    pub fn new(
        set: Box<SetStatement>,
        skip: Option<Box<Expression>>,
        limit: Option<Box<Expression>>,
    ) -> PaginationStatement {
        PaginationStatement { set, skip, limit }
    }
}
