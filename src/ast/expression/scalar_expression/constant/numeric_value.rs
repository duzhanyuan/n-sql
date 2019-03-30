// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub enum NumericValue {
    Integer(i64),
    Float(f64),
}

impl From<i64> for NumericValue {
    fn from(v: i64) -> Self {
        NumericValue::Integer(v)
    }
}

impl From<i32> for NumericValue {
    fn from(v: i32) -> Self {
        NumericValue::Integer(v as i64)
    }
}

impl From<f64> for NumericValue {
    fn from(v: f64) -> Self {
        NumericValue::Float(v)
    }
}

impl From<f32> for NumericValue {
    fn from(v: f32) -> Self {
        NumericValue::Float(v as f64)
    }
}

impl Add for NumericValue {
    type Output = NumericValue;

    fn add(self, other: NumericValue) -> Self::Output {
        use crate::NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li + ri).into(),
                Float(rf) => ((li as f64) + rf).into(),
            },
            Float(lf) => match other {
                Integer(ri) => (lf + (ri as f64)).into(),
                Float(rf) => (lf + rf).into(),
            },
        }
    }
}

impl Sub for NumericValue {
    type Output = NumericValue;

    fn sub(self, other: NumericValue) -> Self::Output {
        use crate::NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li - ri).into(),
                Float(rf) => ((li as f64) - rf).into(),
            },
            Float(lf) => match other {
                Integer(ri) => (lf - (ri as f64)).into(),
                Float(rf) => (lf - rf).into(),
            },
        }
    }
}

impl Div for NumericValue {
    type Output = NumericValue;

    fn div(self, other: NumericValue) -> Self::Output {
        use crate::NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li / ri).into(),
                Float(rf) => ((li as f64) / rf).into(),
            },
            Float(lf) => match other {
                Integer(ri) => (lf / (ri as f64)).into(),
                Float(rf) => (lf / rf).into(),
            },
        }
    }
}

impl Mul for NumericValue {
    type Output = NumericValue;

    fn mul(self, other: NumericValue) -> Self::Output {
        use crate::NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li * ri).into(),
                Float(rf) => ((li as f64) * rf).into(),
            },
            Float(lf) => match other {
                Integer(ri) => (lf * (ri as f64)).into(),
                Float(rf) => (lf * rf).into(),
            },
        }
    }
}
