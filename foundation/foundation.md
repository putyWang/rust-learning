#### [3.1 变量](./variable/src/main.rs)

- 使用 let 关键字进行申明（但不可被二次赋值）；
- 使用 mut 关键字修饰的变量可被二次赋值；
- 常量是使用 const 进行申明的一个不允许改变值的变量（类似与未使用 mut 修饰的变量）；
  - 总是不变的，不允许使用mut进行修饰；
  - 常量必须注明值的类型；
  - 任何作用域中都可以被声明，包括全局作用域；
  - 常量只能被设置为常量表达式，不能为运行时计算出的值；
- 变量的隐藏
  - 通过定义与已定义变量的同名变量可实现对之前定义变量的隐藏；
  - 单一变量可以被重复多次隐藏；
  - 隐藏之后的所有操作都视作对之后的变量所作；
  - 在变量的作用域结束后，才会结束对之前变量的隐藏；
  - 隐藏实际上相当于创建新变量，申明变量类型可以与之前不一致；

#### [3.2 数据类型](./data_type/src/main.rs)

- **标量**（*scalar*）代表一个单独的值；

  - **整型**没有小数部分的数字；

    - 允许使用 `_` 做为分隔符以方便读数；

    - 允许使用字面量表中所有方式标识数据；

    - 类型表

      | **长度** | **有符号** | **无符号** |
      | -------- | ---------- | ---------- |
      | 8-bit    | i8         | u8         |
      | 16-bit   | i16        | u16        |
      | 32-bit   | i32        | u32        |
      | 64-bit   | i64        | u64        |
      | 128-bit  | i128       | u128       |
      | arch     | isize      | usize      |

      -  `isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的 ；

    - 字面量表

      | 数字字面值                    | 示例        |
      | ----------------------------- | ----------- |
      | Decimal (十进制)              | 98_222      |
      | Hex (十六进制)                | 0xff        |
      | Octal (八进制)                | 0o77        |
      | Binary (二进制)               | 0b1111_0000 |
      | Byte (单字节字符)(仅限于`u8`) | b'A'        |

  - **浮点型**（*floating-point numbers*）带小数点的数字；

    - 采用 IEEE-754 标准进行表示；
    - 两种原生：f64（64位双精度浮点数）与f32（32位单精度浮点数）；
    - 默认声明为64位双精度浮点数；

  - **布尔型**（boolean）

    - 两个可能的值：`true` 和 `false`；

  - **字符类型**（char）

    - 使用单引号（'）声明的字面量；
    - 大小为四个字节，代表一个 Unicode 标量值（ 从`U+0000`到`U+D7FF`和`U+E000`到`U+10FFFF`在内的值 ）；

- **复合类型**多个值组合成一个类型；

  - **元组类型**(元素1类型, 元素2类型, 元素3类型...)
    - 将多个其他类型的值组合进一个复合类型的主要方式；
    - 元组长度一旦声明，就不会增大或缩小；
    - 使用圆括号中的逗号分隔的值列表来创建；
    - 元组中数据的访问
      - **解构**：声明新的变量组将元组的所有值匹配对应赋值；
      - **直接访问**：直接使用元组名.index对指定位置的值进行访问（index 从0开始）；
    - **单元元组**：不带任何值的元组；
  - **数组类型**（[数据类型;数组长度]）
    - 数组中的每个元素的类型必须相同；
    - 数组长度是固定的；
    - 使用方括号中的逗号分隔的相同类型值列表来创建；
    - 通过使用方括号包含数组元素类型+分号+数组元素的数量；
    - 也可以使用[元素值;元素个数]的方式创建元素值相同的数组；
    - 数组中元素的访问
      - 使用索引对数据进行访问（数组名[index]， index 索引是从0开始）；
      - 访问无效的索引时会直接抛出异常；

#### [3.3 函数](./foundation/function/src/main.rs)

- 声明
  - 使用 fn 关键字对函数进行声明；
  - 使用  snake case 风格进行命名，字母都是小写并使用下划线分隔单词；
  - 定义格式：fn 函数名(参数列表)；
  - 声明时必须指定指定参数的数据类型；
  - 参数列表中不同参数使用逗号进行分隔；
  - 函数需要返回值时，需要在参数列表括号后加上 -> 返回值类型 定义返回值类型；
- 函数体
  -  函数体由一系列的语句和一个可选的结尾表达式构成 
     - **语句**（*Statements*）是执行一些操作但不返回值的指令；
     - **表达式**（*Expressions*）计算并产生一个值；
     - 表达式结尾不能以分号（;）结尾表示该列产生为返回值

#### [3.4 控制流](./control_stream/src/main.rs)

- **if 表达式**
  - 以 `if` 关键字开头，然后跟一个条件；
  - 条件计算结果需为bool类型值，否则会编译失败；
  - 在条件之后跟随条件为真时执行的语句；
  - 存在多个判断条件分支时，在 if 语句之后 使用else if 进行分支判断；
  - 最后使用 else 关键字处理余量数据；
  - 由于if 语句为表达式，因此可以在let 语句中直接使用；
  - 在 let 语句中使用时，所有分支表达式的结果类型必须一致；
- 循环语句
  - **loop 语句**
    - 重复执行指定语句，直到手动停止（ctrl + c）；
    - 使用 continue 语句可以跳过本次循环中的剩余语句直接进入下一次循环；
    - 使用break语句不仅可以跳出循环，break 语句执行结果还将作为整个loop语句的返回值；
    - 循环嵌套的情况之下，可以通过对循环打标签的方式指定具体跳出哪个循环；
    - 不指定跳出循环标签名，跳出所处最内层循环；
  - **while条件循环**
    - while 条件 循环体 格式进行声明；
    - 在条件满足时，执行循环体中的内容直到条件不满足；
  - **for 循环**
    - for 循环 通常用来遍历集合中的所有元素；
    - for 元素名 in 集合名 {循环体}；

#### [3.5 格式化输出](./format_output/src/main.rs)

- 打印操作由下列 **std::fmt 里面所定义的一系列宏** 来处理
  - **format! 宏**：将格式化文本转化为字符串。
  - **print! 宏**：通过与 format! 类似的方式将格式化文本转化为字符串，随后输出到控制台；
  - **println! 宏**: 与 print! 类似，但输出文本后追加一个换行符；
  - **eprint! 宏**：与 print! 类似，但将文本输出到标准错误；
  - **eprintln! 宏**：与 eprint! 类似，但输出结果追加一个换行符；
  - **write! 宏**：将格式字符串发送到指定的流；
  - **writeln! 宏**：与 write! 类似，但输出结果追加一个换行符；
- 若一个类型需要使用格式化输出时，必须 **至少实现一个可打印的 trait**；
- 有些可打印 trait **提供了自动实现**，如 std::fmt::Debug；
- 其他可打印 trait 则 **必须手动实现**（std::fmt::DIsplay）；