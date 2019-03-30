# N-SQL函数

> N-SQL 内置了很多函数

## N-SQL 字符串函数

函数名 | 描述
-- | --
CONCAT | 将两个或者多个字符串组合在一起
LEFT | 从字符串中提取多个字符（从左侧开始）
LENGTH | 返回字符串的长度
LOWER | 将字符串中的大写字母转为小写字母
LTRIM | 从字符串中删除前导空格
REPLACE | 使用新的子字符串替换字符串中所有出现的子字符串
RIGHT | 从字符串中提取多个字符（从右侧开始）
RTRIM | 从字符串中删除尾随空格
TRIM | 从字符串中删除前导和尾随空格（或其他指定字符）
UPPER | 将字符串中的小写字母转为大写字母

## N-SQL 数学/数值 函数

函数名 | 描述
-- | --
ABS | 返回一个数字的绝对值
AVG | 返回一组数据中的平均值
COUNT | 返回select查询返回的记录数
COS | 返回数字的余弦值
[LOG](/#/zh-CN/functions#log)  | 返回数字的自然对数，或数字的对数到指定的基数
[LOG10](/#/zh-CN/functions#log) | 返回数字的自然对数到10
MAX | 返回一组数据中的最大值
MIN | 返回一组数据中的最小值
POWER | 返回 $x^{y}$（x的y次方） 的值
RANK | 返回一组值中的值的排名
ROUND | 将数字四舍五入到指定的小数位数
SIGN | 返回给定数字的符号
SIN | 返回数字的正弦值
SQRT | 返回数字的平方
SUM | 计算一组数据的总和

## N-SQL 日期函数

函数名 | 描述
-- | --
DAY | 返回指定时间中的天
EXTRACT | 从给定时间中提取年、月、日等部分
HOUR | 返回指定时间中的小时
MINUTE | 返回指定时间中的分钟
MONTH | 返回指定时间中的月份 (有效值为1到12)
NOW | 返回当前时间
SECOND | 返回指定时间中的秒
WEEK | 返回指定时间中的星期
YEAR | 返回指定时间中的年份

## N-SQL 高级函数

函数名 | 描述
-- | --
NVL | 用指定的值替换表达式中的NULL值

## 函数详解

### LOG

> LOG() 函数返回指定数字的自然对数，或者返回特定基数的对数。

- 语法
  > `log(N)`, `log(B, N)`;

   **Note**: `LOG(B,N)` 等同于 `LOG(N) / LOG(B)`。
- 参数说明
  名称 | 数据类型 | 描述
    -- | -- | --
    N | numreic| 数字
    B | numreic| N的基数
- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | 不支持指定基数
- LOG函数在各个数据库的详细支持情况
  - PostgreSQL
    > 基本语法: `LOG(B, N)`。
    1. `LOG(N)` 等同于N-SQL中的`LOG(10, N)`。
  - Oracle
    > 基本语法: `LOG(B, N)`。
    1. Oracle中没有其他的写法了。
  - MySQL
    > 基本语法: `LOG(B, N)`。
    1. `LOG(N)`等同于N-SQL中的`LOG(e, N)`
    2. `LOG10(N)`等同于N-SQL中的`LOG(10, N)`
  - Sql Server
    > 基本语法: `LOG(N, B)`,与其他数据库不太一样，N和B是反过来的。
    1. `LOG(N)`等同于N-SQL中的`LOG(e, N)`
    2. `LOG10(N)`等同于N-SQL中的`LOG(10, N)`
  - SQLite
    > SQLite不支持自定义底数，仅支持以下两种写法。
    1. `LOG(N)`等同于N-SQL中的`LOG(e, N)`
    2. `LOG10(N)`等同于N-SQL中的`LOG(10, N)`

### LOG10() 返回以10为底的对数.

**Note**: 另请参见[LOG()](/#/zh-CN/functions#log)函数。

- 语法
   > `LOG10(N)`;
- 参数
  名称 | 数据类型 | 描述
  -- | -- | --
  N | numreic| 数字
- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$
