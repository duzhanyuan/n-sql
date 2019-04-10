# N-SQL函数

> N-SQL 内置了很多函数

## N-SQL 聚合函数

函数名 | 说明
-- | --
[AVG](/#/zh-CN/functions#avg) | 返回表达式的平均数
[COUNT](/#/zh-CN/functions#count) | 统计给定查询的记录数
[COUNT_IF](.) | TODO
[MAX](/#/zh-CN/functions#max) | 返回集合中的最大值
[MAX_IF](.) | TODO
[MEDIAN](/#/zh-CN/functions#median) | 返回有序集合的中位数
[MEDIAN_IF](.) | TODO
[MIN](/#/zh-CN/functions#min) | 返回集合中的最小值
[MIN_IF](.) | TODO
[PERCENT](/#/functions?id=percentile_cont) | This function is equal to the PERCENTILE_CONT() function.
[PERCENTILE](/#/functions?id=percentile_cont) | This function is equal to the PERCENTILE_CONT() function.
[PERCENTILE_CONT](/#/functions?id=percentile_cont) | Returns an interpolated value in An inverse distribution function that assumes a continuous distribution model.
[PERCENTILE_DISC](/#/functions?id=percentile_disc) | Returns an element from An inverse distribution function that assumes a discrete distribution model
[STDDEV](/#/zh-CN/functions#stddev) | 返回表达式的样本标准偏差
[SUM](/#/zh-CN/functions#sum) | 计算集合中的数据的总和
[SUM_IF](.) | TODO
[VARIANCE](/#/zh-CN/functions#variance) | 返回表达式的方差

## N-SQL 字符串函数

函数名 | 说明
-- | --
[CONCAT](/#/zh-CN/functions#concat) | 将两个或者多个字符串组合在一起
[LEFT](/#/zh-CN/functions#left) | 从字符串中提取多个字符（从左侧开始）
[LENGTH](/#/zh-CN/functions#length) | 返回字符串的长度
[LOWER](/#/zh-CN/functions#lower) | 将字符串中的大写字母转为小写字母
[LTRIM](/#/zh-CN/functions#ltrim) | 从字符串中删除前导空格
[REPLACE](/#/zh-CN/functions#replace) | 使用新的子字符串替换字符串中所有出现的子字符串
[RIGHT](/#/zh-CN/functions#right) | 从字符串中提取多个字符（从右侧开始）
[RTRIM](/#/zh-CN/functions#rtrim) | 从字符串中删除尾随空格
[TRIM](/#/zh-CN/functions#trim) | 从字符串中删除前导和尾随空格（或其他指定字符）
[UPPER](/#/zh-CN/functions#upper) | 将字符串中的小写字母转为大写字母

## N-SQL 数学/数值 函数

函数名 | 说明
-- | --
[ABS](/#/zh-CN/functions#abs) | 返回一个数字的绝对值
[CEIL](/#/zh-CN/functions#ceil) | 返回大于或等于指定数字的最小整数
[CEILING](/#/zh-CN/functions#ceil) | 同[CEIL()](/#/functions#ceil)
[COS](/#/zh-CN/functions#cos) | 返回数字的余弦值
[DENSE_RANK](/#/zh-CN/functions#dense_rank) | ss
[FLOOR](/#/zh-CN/functions#floor) | ss
[LOG](/#/zh-CN/functions#log)  | 返回数字的自然对数，或数字的对数到指定的基数
[LOG10](/#/zh-CN/functions#log10) | 返回数字的自然对数到10
[POW](/#/zh-CN/functions#pow) | ss
[POWER](/#/zh-CN/functions#power) | 返回 $x^{y}$（x的y次方） 的值
[RANK](/#/zh-CN/functions#rank) | 返回一组值中的值的排名
[ROUND](/#/zh-CN/functions#round) | 将数字四舍五入到指定的小数位数
[SIGN](/#/zh-CN/functions#sign) | 返回给定数字的符号
[SIN](/#/zh-CN/functions#sin) | 返回数字的正弦值
[SQRT](/#/zh-CN/functions#sqrt) | 返回数字的平方
[TAN](/#/zh-CN/functions#tan) | ss

## N-SQL 日期函数

函数名 | 说明
-- | --
[DAY](/#/zh-CN/functions#day) | 返回指定时间中的天
[DAY_ADD](/#/zh-CN/functions#day_add) | s
[DAY_DIFF](/#/zh-CN/functions#day_diff) | ss
[DAY_SUB](/#/zh-CN/functions#day_sub) | ss
[HOUR](/#/zh-CN/functions#hour) | 返回指定时间中的小时
[HOUR_ADD](/#/zh-CN/functions#hour_add) | ss
[HOUR_DIFF](/#/zh-CN/functions#hour_diff) | ss
[HOUR_SUB](/#/zh-CN/functions#hour_sub) | ss
[MINUTE](/#/zh-CN/functions#minute) | 返回指定时间中的分钟
[MINUTE_ADD](/#/zh-CN/functions#minute_add) | ss
[MINUTE_DIFF](/#/zh-CN/functions#minute_diff) | ss
[MINUTE_SUB](/#/zh-CN/functions#minute_sub) | ss
[MONTH](/#/zh-CN/functions#month) | 返回指定时间中的月份 (有效值为1到12)
[MONTH_ADD](/#/zh-CN/functions#month_add) | ss
[MONTH_DIFF](/#/zh-CN/functions#month_diff) | ss
[MONTH_SUB](/#/zh-CN/functions#month_sub) | ss
[NOW](/#/zh-CN/functions#now) | 返回当前时间
[SECOND](/#/zh-CN/functions#second) | 返回指定时间中的秒
[SECOND_ADD](/#/zh-CN/functions#second_add) | ss
[SECOND_DIFF](/#/zh-CN/functions#second_diff) | ss
[SECOND_SUB](/#/zh-CN/functions#second_sub) | ss
[WEEK](/#/zh-CN/functions#week) | 返回指定时间中的星期
[WEEK_ADD](/#/zh-CN/functions#week_add) | ss
[WEEK_DIFF](/#/zh-CN/functions#week_diff) | ss
[WEEK_SUB](/#/zh-CN/functions#week_sub)  | ss
[YEAR](/#/zh-CN/functions#year) | 返回指定时间中的年份
[YEAR_ADD](/#/zh-CN/functions#year_add) | ss
[YEAR_DIFF](/#/zh-CN/functions#year_diff) | ss
[YEAR_SUB](/#/zh-CN/functions#year_sub) | ss

## N-SQL 高级函数

函数名 | 说明
-- | --
[COALESCE](/#/zh-CN/functions#coalesce) | s
IFNULL | s
NULLIF | s
NVL | 用指定的值替换表达式中的NULL值

## 函数详解

### ABS

> ABS()函数返回的指定数字的绝对值。

- 语法
  > ABS(N);
- 参数
  名称 | 数据类型 | 描述
  -- | -- | --
  N | numeric | 需要计算绝对值的数字
- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### AVG

> AVG() 函数返回一组数值的平均值。

- 语法
  > AVG([ALL | DISTINCT] expression)

- 参数说明
  名称 | 描述
  -- | --
  ALL | 使用所有的值
  DISTICNT | 返回去重复后的值的平均数
  expression | expression是表达式，表达式可以是单个常量，变量，标量函数或者字段名。表达式的结果是数字或者与数字相似的数据类型， 不能是聚合函数和子查询。

- 支持
   数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### CEIL

TODO

### COALESCE

TODO

### CONCAT

TODO

### COS

TODO

### COUNT

TODO

### DAY

TODO

### DAY_ADD

TODO

### DAY_DIFF

TODO

### DAY_SUB

TODO

### DECODE

TODO

### DENSE_RANK

TODO

### FLOOR

TODO

### FORMAT

TODO

### HOUR

TODO

### HOUR_ADD

TODO

### HOUR_DIFF

TODO

### HOUR_SUB

TODO

### LEFT

TODO

### LENGTH

TODO

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

### LTRIM

TODO

### LOWER

TODO

### MAX

TODO

### MAX_IF

TODO

### MEDIAN

TODO

### MIN

TODO

### MIN_IF

TODO

### MINUTE

TODO

### MINUTE_ADD

TODO

### MINUTE_DIFF

TODO

### MINUTE_SUB

TODO

### MONTH

TODO

### MONTH_ADD

TODO

### MONTH_DIFF

TODO

### MONTH_SUB

TODO

### NOW

TODO

### NVL

TODO

### PERCENTILE_CONT

TODO

### PERCENTILE_DISC

> PERCENTILE_DISC（）函数是一个逆分布函数，它假设一个离散的分布模型。它需要百分位值和排序规范，并从集合中返回一个元素。在计算中忽略空值。

- 语法
  > `percentile_dist(expr: [:number], p:float)`;`percentile_dist(expr: [:number], p:float, asc | desc)`

- 参数
  名称 | 说明
  -- | --
  expr | 表达式的值要能够进行排序
  p | 值必须在0和1之间， 因为它是一个分位值

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{red} {\times}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{red} {\times}$

### POW

> `POW()`函数返回一个数字的n次幂的结果。

- 语法
  > `POW(x, y)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  x | numeric | 必填。一个数字(底数)
  y | numeric | 必填。一个数字(指数)

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### RANK

> `RANK()` 函数是计算一组数据的排列登记，返回的类型是数字。

- 语法
  `rank(expression, ASC | DESC)`

- 支持
   数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{red} {\times}$

### REPLACE

> `REPLACE()`函数使用新的子字符串替换字符串中所有出现的子字符串。

**Note**: 替换字符串搜索是大小写敏感的。

- 语法
  > `REPLACE(string, from_string, new_string)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  string | string |必填。 原始字符串
  from_string | string | 必填。 需要被替换的子字符串
  new_string | string | 必填。 需要被替换进去的新的子字符串。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### REVERSE

> `REVERSE()`函数是将一个字符串的反战颠倒返回结果。

- 语法
  > `REVERSE(string)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  string | string | 必填。 需要反转的字符串。

- 支持
   数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### RIGHT

> `RIGHT()`函数是从右往左截取指定长度的字符串。

- Syntax
  > `RIGHT(text, number_of_chars)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  text | string | 必填。 将要被截取的字符串
  number_of_chars  | int | 必填。 需要截取的字符数。如果这个歌参数大于被截取的字符串长度，将返回这一整个字符串。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

### ROUND

> `ROUND()`返回一个数值，该数值是按照指定的小数位数进行四舍五入运算的结果。

**Note**: 另请查看`FLOOR()`, `CEIL()` 和 `CEILING()` 函数。

- 语法
  > `ROUND(number, decimals, operation)`

- 参数
  名称 | 数据类型 | 说明
  number | numeric | 必填。 需要进行四舍五入的数字
  decimals | int | 可选。 保留小数的位数。 如果省略, 将返回正数。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### RTRIM

> RTRIM()函数移除字符串右边的空白符或者指定的符号

- 语法
  > `RTRIM(text, trim_text)`; `TRIM_END(text, trim_text)`

- 参数
  名称 | 数据类型 | 描述
  -- | -- | --
  text | string | 必填。 需要修改的自返还。
  trim_text| 可选. 需要从右边移除的字符(如果省略则移除空白符)。

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | Only Support trim whitespace
  SQlite | $\color{green} {\checkmark}$

### SECOND

> SECOND()函数返回一个时间中的秒的部分。

- 语法
  > `SECOND(date)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  date | date to timestamp | 必填。需要获取秒部分的时间。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{red}{\times}$

### SECOND_ADD

> SECOND_ADD()函数是添加指定的秒数到一个日期上。

- 语法
  > `SECOND_ADD(date, number)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  date | date or timestamp | 必填. 将被增加描述的时间。
  number | int | 必填. 需要加上的秒数。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

### SECOND_DIFF

> SECOND_DIFF()返回两个时间点的总秒数。

- Syntax
  > `SECOND_DIFF(start, end)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  start | date or timestamp | 必填。开始时间
  end | date or timestamp | 必填。结束时间

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

### SECOND_SUB

> SECOND_SUB()函数将一个时间减去指定的秒数。

- 语法
  > `SECOND_SUB(date, number)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  date | datetime or timestamp | 必填. 需要被修改的日期。
  number | int | 需要减去的秒数。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{orange} {\checkmark}$
  Oracle | $\color{orange} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{orange} {\checkmark}$

### SIGN

> SIGN()函数返回一个数字的符号.

- 提示
  该函数将返回一下的一种情况:
  - 如果数字大于0，返回1。
  - 如果数字等于0，返回0
  - 如果数字小鱼0，返回-1

- 语法
  > `SIGN(number)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  number | numeric | 必填。需要计算符号的数字。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### SIN

> SIN() 函数返回一个数值的正弦值。

- 语法
  > `SIN(number)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  number | numeric | 必填。一个数值。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### SQRT

> SQRT()函数是将一个非负数值开平方根的。

- 语法
  > `SQRT(number)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  number | numeric | 必填。需要开平方根的数字。必须是大于0的。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### STDDEV

> STDDEV()函数返回表达式的标准差，如果没有匹配的行将返回NULL。

- 语法
  > `STDDEV([ALL | DISTINCT] expression)`

- 参数
  名称 | 描述
  -- | --
  ALL | 应用所所有的值
  DISTINCT | 计算去重复后的标准差
  expression | expression是表达式，表达式可以是单个常量，变量，标量函数或者字段名。表达式的结果是数字或者与数字相似的数据类型， 不能是聚合函数和子查询。

- Support
  Database | Support
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{red}{\times}$

### SUBSTR

- 语法
  > `SUBSTR(text, start, length)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  text | string | 必填。将被截取的字符串
  start | int | 必填。开始位置， 既可以是正数也可以是父属。如果是正数，这个函数开始开始的位置截取字符串。如果是负数，这个函数从结尾开始截取数字
  end | int | 可选。截取字符长度，如果省略， 所有的字符串将被截取返回。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{orange} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### SUM

> SUM()函数是计算给定的一组数字的综合。

- 语法
  > `SUM([ALL | DISTINCT] expression)`

- 参数说明
  名称 | 描述
  -- | --
  ALL | 使用所有的值
  DISTICNT | 返回去重复后的值的总和
  expression | expression是表达式，表达式可以是单个常量，变量，标量函数或者字段名。表达式的结果是数字或者与数字相似的数据类型， 不能是聚合函数和子查询。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### TAN

> TAN()函数返回了一个数值的正切值。

- 语法
  > `TAN(N)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  N | numeric | 数字

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### TRIM

> TRIM()函数移除字符串首部和尾部的空白字符(或者其他指定的字符)。

- 语法
  > `TRIM(S)`; `TRIM(S,T)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  S | string | 必填。将被修改的字符串
  T | string | 可选。将被移除的字符串(如果省略则表示去除空白字符)

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{orange} {\checkmark}$
  Sql Server | 只支持移除空白字符
  SQlite | $\color{green} {\checkmark}$

### UPPER

> UPPER()函数是将给定的字符串中的小写字母转换为大写字母。

**Note**: 另请参阅[LOWER()](/#/zh-CN/functions#lower)函数。

- 语法
  > `UPPER(S)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  N | string | 需要转为大写的字符串

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### VARIANCE

> VARIANCE()是计算一组数据的标准差

- 语法
  `VARIANCE([ALL | DISTINCT] expression)`

- 参数说明
  名称 | 描述
  -- | --
  ALL | 使用所有的值
  DISTICNT | 返回去重复后的值的平均数
  expression | expression是表达式，表达式可以是单个常量，变量，标量函数或者字段名。表达式的结果是数字或者与数字相似的数据类型， 不能是聚合函数和子查询。

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{green} {\checkmark}$
  Oracle | $\color{green} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{green} {\checkmark}$

### YEAR

> YEAR()函数返回一个日期中的年。

- 语法
  > `YEAR(D)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  D | date or timestamp | 将要被提取年部分的日期

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{green} {\checkmark}$
  Sql Server | $\color{green} {\checkmark}$
  SQlite | $\color{red} {\times}$

### YEAR_ADD

> YEAR_ADD() 函数是将一个时间加上指定的年数。

- 语法
  > `YEAR_ADD(D, N)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  D | date or timestamp | 将被修改的时间
  N | int | 需要加上的年数

- 支持
  数据库 | 数据支持情况
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$

### YEAR_DIFF

> YEAR_DIFF()函数返回两个日期之间的年数差。

- 语法
  > `YEAR_DIFF(start, end)`

- 参数
  名称 | 数据类型 | 说明
  -- | -- | --
  start | date or timestamp | 开始时间
  end | date or timestamp | 结束时间

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$


### YEAR_SUB

> YEAR_SUB()函数将一个时间减去指定的年数。

- 语法
  > `YEAR_SUB(D, N)`

- 参数说明
  名称 | 数据类型| 说明
  -- | -- | --
  D | date or timestamp| 将要被更改的时间
  N | int | 将要减去的年数

- 支持
  数据库 | 支持情况
  -- | --
  PostgreSQL | $\color{Orange} {\checkmark}$
  Oracle | $\color{Orange} {\checkmark}$
  MySQL | $\color{Orange} {\checkmark}$
  Sql Server | $\color{Orange} {\checkmark}$
  SQlite | $\color{Orange} {\checkmark}$