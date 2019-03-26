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
#[test]
#[case("select * FROM student", NSQL, "select * from student")]
#[case("select name FROM student", NSQL, "select name from student")]
#[case(
    "select nvl(name,     'No Name') FROM student",
    NSQL,
    "select nvl(name, 'No Name') from student"
)]
#[case(
    "select nvl(name,     'No Name') as x FROM student",
    NSQL,
    "select nvl(name, 'No Name') as x from student"
)]
#[case(
    "select nvl(name,     'No Name')       x FROM student",
    NSQL,
    "select nvl(name, 'No Name') as x from student"
)]
#[case("select name, age FROM student", NSQL, "select name, age from student")]
#[case(
    "select t.name, t.age FROM student t",
    NSQL,
    "select t.name, t.age from student as t"
)]
#[case(
    "select name FROM student where age > 3",
    NSQL,
    "select name from student where age > 3"
)]
#[case(
    "select count(id), gender FROM student group by gender",
    NSQL,
    "select count(id), gender from student group by gender"
)]
#[case(
    "select count(id), gender FROM student group by gender having count(id) > 3",
    NSQL,
    "select count(id), gender from student group by gender having count(id) > 3"
)]
#[case(
    "select count(id) FROM student order by name",
    NSQL,
    "select count(id) from student order by name"
)]
#[case(
    "select count(id), name FROM student where a>3 group by name having count(id) > 1 order by age skip 2 limit 3",
    NSQL,
    "select count(id), name from student where a > 3 group by name having count(id) > 1 order by age skip 2 limit 3"
)]
fn test_select(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}
#[theory]
#[test]
#[case(
    "select name from student where gender ='男' intersect select name from student where gender ='女'",
    NSQL,
    "select name from student where gender = '男' intersect select name from student where gender = '女'"
)]
fn test_intersect(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[theory]
#[test]
#[case(
    "select count(id) FROM student skip 3",
    NSQL,
    "select count(id) from student skip 3"
)]
#[case(
    "select count(id) FROM student skip 3",
    Oracle,
    "select count(id) from student offset 3 rows"
)]
#[case(
    "select count(id) FROM student skip 1",
    Oracle,
    "select count(id) from student offset 1 row"
)]
#[case("select * from student skip 1", NSQL, "select * from student skip 1")]
//#[case("select * from student skip 1", PostgreSQL, "select * from student offset 1")]
//#[case("select * from student skip 1", MySQL, "select * from student limit 1")]
//#[case("select * from student skip 1", SqlServer, "select * from student order by 1 offset 1 row")]
//#[case("select * from student skip 2", SqlServer, "select * from student order by 1 offset 2 rows")]
//#[case("select * from student skip 1", SQLite, "select * from student limit -1 offset 1")]
fn test_select_with_skip(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[theory]
#[test]
#[case(
    "select count(id) FROM student limit 3",
    NSQL,
    "select count(id) from student limit 3"
)]
#[case(
    "select count(id) FROM student limit 3   + 2",
    NSQL,
    "select count(id) from student limit 3 + 2"
)]
#[case(
    "select count(id) FROM student limit 3",
    Oracle,
    "select count(id) from student fetch first 3 rows only"
)]
#[case(
    "select count(id) FROM student limit 1",
    Oracle,
    "select count(id) from student fetch first 1 row only"
)]

//#[case("select * from student limit 1", NSQL, "select * from student limit 1")]
//#[case("select * from student limit 1", PostgreSQL, "select * from student limit 1")]
//#[case("select * from student limit 1", Oracle, "select * from student fetch first 1 row only")]
//#[case("select * from student limit 2", Oracle, "select * from student fetch first 2 rows only")]
//#[case("select * from student limit 1", MySQL, "select * from student limit 0, 1")]
//#[case("select * from student limit 1", SqlServer, "select * from student order by 1 offset 0 row fetch next 1 row only")]
//#[case("select * from student limit 2", SqlServer, "select * from student order by 1 offset 0 row fetch next 2 rows only")]
//
//#[case("select * from student skip 3 limit 2", NSQL, "select * from student skip 3 limit 2")]
//#[case("select * from student skip 3 limit 2", PostgreSQL, "select * from student offset 3 limit 2")]
//#[case("select * from student skip 3 limit 2", Oracle, "select * from student offset 3 rows fetch first 2 rows only")]
//#[case("select * from student skip 3 limit 2", MySQL, "select * from student limit 3, 2")]
//#[case("select * from student skip 3 limit 2", SqlServer, "select * from student order by 1 offset 3 rows fetch next 2 rows only")]
//#[case("select * from student skip 3 limit 2", SQLite, "select * from student limit 2 offset 3")]
fn test_select_with_limit(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[theory]
#[test]
#[case("select now()", NSQL, "select now()")]
#[case("select now() from dual", NSQL, "select now()")]
//#[case("select now() from dual", Oracle, "select now() from dual")]
fn test_select_from_dual(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[test]
fn test_select_from_sub_query() {
    test_statement(
        NSQL,
        "SELECT NOW() from (select * from student) t",
        "select now() from (select * from student) as t",
    );
}

#[test]
fn test_select_case_when() {
    test_statement(
        NSQL,
        "select case when score >= 85 then 'A' when score<85 and score >=60 then 'B' else 'C' end level from student",
        "select case when score >= 85 then 'A' when score < 85 and score >= 60 then 'B' else 'C' end as level from student",
    );
}

#[theory]
#[test]
#[case(
    "SELECT name from (select * from tree) t join tree t2 on t2.parent_id = t.id",
    NSQL,
    "select name from (select * from tree) as t join tree as t2 on (t2.parent_id = t.id)"
)]
fn test_select_join(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[theory]
#[test]
#[case(
    "select name from student where gender ='男' union select name from student where gender ='女'",
    NSQL,
    "select name from student where gender = '男' union select name from student where gender = '女'"
)]
#[case(
    "select name from student where gender ='男' union select name from student where gender ='女' skip 1 limit 2",
    NSQL,
    "select name from student where gender = '男' union select name from student where gender = '女' skip 1 limit 2"
)]
#[case(
    "(select name from student where gender ='男' skip 2) union select name from student where gender ='女' skip 1 limit 2",
    NSQL,
    "(select name from student where gender = '男' skip 2) union select name from student where gender = '女' skip 1 limit 2"
)]
#[case(
    "select name from student where gender ='男' union all select name from student where gender ='女'",
    NSQL,
    "select name from student where gender = '男' union all select name from student where gender = '女'"
)]
fn test_union(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[test]
fn test_union_err() {
    let statement = StatementParser::new().parse(Lexer::new("select name from student where gender ='男' skip 2 union select name from student where gender ='女' skip 1 limit 2").tokenizer());
    assert_eq!(true, statement.is_err());
}

#[theory]
#[test]
#[case(
    "select name from student where gender ='男' except select name from student where gender ='女'",
    NSQL,
    "select name from student where gender = '男' minus select name from student where gender = '女'"
)]
#[case(
    "select name from student where gender ='男' minus select name from student where gender ='女'",
    NSQL,
    "select name from student where gender = '男' minus select name from student where gender = '女'"
)]
fn test_minus(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}
