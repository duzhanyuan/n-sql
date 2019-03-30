// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ast::Expression;
use crate::ast::Identifier;
use crate::ast::PredicateExpression;
use crate::ast::SortingDirection;
use crate::ast::TableView;

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub select_type: Option<SelectType>,
    pub elements: Vec<SelectElement>,
    pub tables: Option<Vec<Box<TableView>>>,
    pub predicate: Option<Box<PredicateExpression>>,
    pub group: Option<GroupElements>,
    pub sorts: Option<Vec<SortingElement>>,
}

impl SelectStatement {
    pub fn new(
        select_type: Option<SelectType>,
        elements: Vec<SelectElement>,
        tables: Option<Vec<Box<TableView>>>,
        predicate: Option<Box<PredicateExpression>>,
        group: Option<GroupElements>,
        sorts: Option<Vec<SortingElement>>,
    ) -> SelectStatement {
        SelectStatement {
            select_type,
            elements,
            tables,
            predicate,
            group,
            sorts,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SelectElement {
    Expression(Box<Expression>, Option<Identifier>),
    Asterisk(Option<Identifier>),
}

#[derive(Clone, Debug)]
pub struct GroupElements {
    pub elements: Vec<Box<Expression>>,
    pub having: Option<Box<PredicateExpression>>,
}

impl GroupElements {
    pub fn new(
        elements: Vec<Box<Expression>>,
        having: Option<Box<PredicateExpression>>,
    ) -> GroupElements {
        GroupElements { elements, having }
    }
}
#[derive(Clone, Debug)]
pub struct SortingElement {
    pub expr: Box<Expression>,
    pub direction: Option<SortingDirection>,
}

impl SortingElement {
    pub fn new(expr: Box<Expression>, direction: Option<SortingDirection>) -> SortingElement {
        SortingElement { expr, direction }
    }
}
#[derive(Clone, Debug, Copy)]
pub enum SelectType {
    All,
    Distinct,
    Unique,
}

mod select_builder {
    struct Builder {}
    pub trait IHasFrom {
        fn from() -> Box<IKeySelect>;
    }
    pub trait IKeySelect {}
    pub trait IkeyFrom {}
    fn select() -> Box<IKeySelect> {
        unimplemented!()
    }
}
