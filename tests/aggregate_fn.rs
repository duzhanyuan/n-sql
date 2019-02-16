// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod common;

test_init!();

#[theory]
#[case("avg(a)", NSQL, "avg(a)")]
#[case("avg(distinct a)", NSQL, "avg(distinct a)")]
#[case("count(a)", NSQL, "count(a)")]
#[case("count(unique a)", NSQL, "count(unique a)")]
#[case("sum(all a)", NSQL, "sum(all a)")]
#[case("sum(a)", NSQL, "sum(a)")]
#[case("stddev(a)", NSQL, "stddev(a)")]

#[case("avgif(gender = '男', a)", NSQL, "avgif(gender = '男', a)")]
#[case("avgif(gender = '男',distinct a)", NSQL, "avgif(gender = '男', distinct a)")]
#[case("countif(gender = '男',a)", NSQL, "countif(gender = '男', a)")]
#[case("countif(gender = '男',unique a)", NSQL, "countif(gender = '男', unique a)")]
#[case("sumif(gender = '男',all a)", NSQL, "sumif(gender = '男', all a)")]
#[case("sumif(gender = '男',a)", NSQL, "sumif(gender = '男', a)")]
#[case("stddevif(gender = '男',a)", NSQL, "stddevif(gender = '男', a)")]

#[case("avgif(gender = '男', a)", PostgreSQL, "avg(case when gender = '男' then a else null end)")]
#[case("avgif(gender = '男',distinct a)", PostgreSQL, "avg(distinct case when gender = '男' then a else null end)")]
#[case("countif(gender = '男',a)", PostgreSQL, "count(case when gender = '男' then a else null end)")]
#[case("countif(gender = '男',unique a)", PostgreSQL, "count(unique case when gender = '男' then a else null end)")]
#[case("sumif(gender = '男',all a)", PostgreSQL, "sum(all case when gender = '男' then a else null end)")]
#[case("sumif(gender = '男',a)", PostgreSQL, "sum(case when gender = '男' then a else null end)")]
#[case("stddevif(gender = '男',a)", PostgreSQL, "stddev(case when gender = '男' then a else null end)")]


fn test(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right );
}

#[test]
fn test_avg2(){
    test_predicate(NSQL,"avg(a) >9", "avg(a) > 9");
}




