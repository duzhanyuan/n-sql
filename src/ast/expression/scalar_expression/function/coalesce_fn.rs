// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ast::Expression;

#[derive(Clone, Debug)]
pub struct CoalesceFn {
    pub items: Vec<Box<Expression>>,
}

impl CoalesceFn {
    pub fn new(items: Vec<Box<Expression>>) -> CoalesceFn {
        CoalesceFn { items }
    }
}
