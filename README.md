## 从零开始学习Rust

### 1.配置rust开发环境

#### 1.1 windows安装

- 配置c++环境

  - 使用visual Studio 配置
    - **下载地址**：[Visual Studio 2022](https://visualstudio.microsoft.com/downloads/)；
    - **工作负载**（workload）
      - 使用 C++ 的桌面开发（Desktop Development with C++）；
      - Windows 10（或 11）SDK；
      - 英语语言包，以及其他你所需要的语言包；
  - 手动配置（ MinGW64 ）
    - **下载地址**：[MinGW-w64 - for 32 and 64 bit Windows - Browse /mingw-w64/mingw-w64-release at SourceForge.net](https://sourceforge.net/projects/mingw-w64/files/mingw-w64/mingw-w64-release/) ；
    - **下载**： MinGW-W64 Online Installer；
    - 安装时系统设置
      - **版本（version）**：按需选取，默认选择最新版本；
      - **系统架构（architecture）**：64位系统配置x86-64，32位系统设置i686；
      - **操作系统接口协议（Threads）**：windows系统选择win32， 其他系统选择posix；
      - **异常处理模型（Exception）**：seh效率高不支持32位系统， SJLJ 兼容32位与64位但效率低于seh，dwarf只支持32位系统；
      - **备注**：
        - "the file has been downloaded incorrectly!"报错解决 时直接选择对应的离线安装包解压；
    - 环境变量配置
      - path中添加安装文件夹下的bin目录；
    - 安装完毕后使用g++ --version验证是否成功安装
- 安装配置rustUp

  - 下载地址： [Install Rust - Rust Programming Language (rust-lang.org)](https://www.rust-lang.org/tools/install) ；

  - 安装配置

    - These components can be acquired through a Visual Studio installer

      - 由于MinGW64  工具链构建的C++是用gnu 因此选择**选项3** (Don't install the prerequisites)；
- Current installation options:
  
  - 选择**选项2**自定义安装( Customize installation)  ；
      - 默认安装使用的 x86_64-pc-windows-msvc 与 MinGW64  使用的gnu不一致，需要修改为 x86_64-pc-windows-gnu；
      - 其他选项根据自己需求修改；
    - 安装完毕后使用rustc --version 验证是否安装成功；
- 配置开发软件（vscode）
  - 插件
    - rust-analyzer： 实时编译和分析 Rust 代码，提示错误，同时对类型进行标注 ；
    - Rust：Rust语言服务器；
    - rust syntax： 为代码提供语法高亮 ；
    - crates：依赖分析；
    - better toml： Rust 使用 toml 做项目的配置管理 ；
    - rust test lens： 快速运行 Rust 测试 ；
    - Tabnine：基于 AI 的自动补全，可以帮助你更快地撰写代码； 

### 2.cargo项目

#### 2.1 命令

- cargo new 项目名 创建项目；
  - 使用 **--lib** 参数创建项目的同时，为lib.rs自动生成单元测试；
- 项目根目录运行命令
  - cargo run 运行项目；
    - 使用 **-- 命令行参数** 参数向程序中传入命令行参数（多个命令行参数使用 **空格** 进行分隔）；
    - 使用 **> 标准错误文件名** 设置标准错误文件；
  - cargo test 对项目进行自动化测试；
    - 默认使用 **多线程并发** 的方式进行所有测试；
    - 使用 **--help** 提示 cargo test 所有可选参数；
    - 使用 **-- --test-threads=线程数** 参数指定测试可使用的线程数；
    - 使用 **-- -- show-output ** 参数表明测试会打印 println! 的输出；
    - 在命令后加上 **部分或完整测试方法名** 运行所有包含指定字段值的测试；
    - 使用 **--test 集成测试文件名** 的方式运行指定集成测试文件名；
  - cargo build 构建项目
    - 不带参数时使用默认参数构建项目；
    - **-- 配置名** 采用 profile.配置名中的配置对项目进行构建；

#### 2.2 配置文件（Cargo.toml）

-  package： 设置项目的相关信息；

  - name：项目名（必填）；
    - 其它项目引用我们的 `package` 时，会使用该 `name`；
    - 编译出的可执行文件(bin target)的默认名称；
    - 限制
      - **使用 `cargo new` 或 `cargo init` 创建时**，`name` 还会被施加额外的限制；
      - 要发布到 `crates.io`时 `name` 需使用 `ASCII` 码，不能使用已经被使用的名称；
  - version：项目版本（必填）；
    - 符合 `"x.y.z"` 的形式，其中 `x` 被称为主版本(major), `y` 被称为小版本 `minor` ，而 `z` 被称为 补丁 `patch` ;
    - 使用标准的 `x.y.z` 形式的版本号，例如 `1.0.0` 而不是 `1.0`
    - 在版本到达 `1.0.0` 之前，怎么都行，但是如果有破坏性变更( breaking changes )，需要增加 `minor` 版本号；
    - 在 `1.0.0` 之后，如果发生破坏性变更，需要增加 `major` 版本号；
    - 在 `1.0.0` 之后，如果要添加新的 `pub` 结构体、特征、类型、函数、方法等对象时，增加 `minor` 版本号；
  - authors：作者；
  - edition： 指定项目所使用的rust版本；
  - rust-version： 用于说明你的项目支持的最低 Rust 版本(编译器能顺利完成编译) ；
  - description：项目简介；
  - documentation：项目文档地址；
  - readme：指向readme文件；
  - homepage：主页地址；
  - repository：设置项目源代码仓库地址；
  - license： 用于描述项目所遵循的开源协议；
  - license-file：用于指定包含开源协议的文件所在的路径(相对于 `Cargo.toml`) ；
  - keywords：项目关键字列表，用于cargo.io检索时使用；
  - categories：用于描述项目所属的类别；
  - workspace：用于配置当前项目所属的工作空间；
  - build：用于指定位于项目根目录中的构建脚本；
  - links：用于指定项目链接的本地库的名称；
  - exclude和include：这两个字段可以用于显式地指定想要包含在外或在内的文件列表；
  - publish：用于防止项目因为失误被发布到 crates.io 等注册服务上；
  
-  profile.* 
  
  -  项目构建相关配置信息；
  -  **opt-level** 代码优化程度（0-无优化，1-基础优化， 2-部分优化，3-全部优化，"s"-针对二进制大小进行优化，"z"-针对二进制大小进行优化的同时关闭循环矢量化）；
  
- metadata： 用户自定义的提供给外部工具的配置文件；
  
  - default-run：使用 cargo run 来运行项目时，该命令会使用默认的二进制可执行文件作为程序启动入口；
  
- badges： 用于指定项目当前的状态，展示在 `crates.io` 的项目主页；

  -  maintenance 是项目的当前维护状态，可能会被其它注册服务所使用；
     -  status 字段时必须的
        -  actively-developed: 正在增加新功能及修复bug；
        -  passively-maintained: 项目停止添加新功能，项目维护者可能会回答你提出的issue；
        -  as-is: 项目已经结束且功能已达预期，维护者不准备继续开发和提供支持；
        -  experimental: 作者希望同大家分享，但是还不准备满足任何人的特殊要求；
        -  looking-for-maintainer: 维护者希望将项目转移给新的维护者；
        -  deprecated: 不再推荐使用该项目，同时说明原因以及推荐的替代项目；
        -  none:  不显示任何 badge；

- dependencies：引用三方依赖包；

  -  依赖包版本设置
     -  ^ 可以指定一个版本号范围，然后会使用该范围内的最大版本号来引用对应的包；
     -  ~ 指定了最小化版本；
     -  `*` 通配符允许将 `*` 所在的位置替换成任何数字；
     -  使用比较符的方式来指定一个版本号范围或一个精确的版本号， 同时还能使用比较符进行组合，并通过逗号分隔；

  - 从其它注册服务引入依赖包
    - 第一种：
      -  在 crates.io 之外添加新的注册服务；
      -  在引用依赖包时要指定注册服务；
    - 第二种
      -  直接使用新注册服务来替代默认的 crates.io；
  - 引入 git 仓库作为依赖包；
  - 通过路径引入本地依赖包；
  - 根据特定的平台来引入依赖；

  ```toml
  [package]
  name = "rust-test"
  version = "0.1.0"
  edition = "2021"
  
  # 项目测试环境配置
  [profile.dev]
  opt-level = 0
  # 项目release环境配置
  [profile.release]
  opt-level = 3
  
  [registries]
  ustc = { index = "https://mirrors.ustc.edu.cn/crates.io-index/" }
  
  [badges]
  maintenance = { status = "..." }
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  # ^1:[>=1.0.0, <2.0.0]
  # *     := >=0.0.0
  # 1.*   := >=1.0.0, <2.0.0
  # 1.2.* := >=1.2.0, <1.3.0
  # >= 1.2, < 1.5
  # time = {  registry = "ustc" }
  time = "0.1.12"
  # 引入 git 仓库作为依赖包
  regex = { git = "https://github.com/rust-lang/regex" }
  
  # 通过路径引入本地依赖包
  # 以下路径也可以
  # hello_utils = { path = "./hello_utils" }
  # hello_utils = { path = "../hello_world/hello_utils" }
  hello_utils = { path = "hello_utils" }
  
  # 引入windows操作平台依赖
  [target.'cfg(windows)'.dependencies]
  winhttp = "0.4.0"
  # 引入unix操作平台依赖
  [target.'cfg(unix)'.dependencies]
  openssl = "1.0.1"
  # 引入x86架构平台依赖
  [target.'cfg(target_arch = "x86")'.dependencies]
  native = { path = "native/i686" }
  # 引入x86_64架构平台依赖
  [target.'cfg(target_arch = "x86_64")'.dependencies]
  native = { path = "native/x86_64" }
  
  # 直接使用新注册服务来替代默认的 crates.io
  [source.crates-io]
  replace-with = 'ustc'
  
  [source.ustc]
  registry = "git://mirrors.ustc.edu.cn/crates.io-index"
  ```

### 3.基础知识

#### [3.1 变量](./foundation/variable/src/main.rs)

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

#### [3.2 数据类型](./foundation/data_type/src/main.rs)

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

#### [3.2 函数](./foundation/function/src/main.rs)

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

#### [3.3 控制流](./foundation/control_stream/src/main.rs)

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

### 4.所有权

#### [4.1 基础定义](./scope/base/src/main.rs)

- 基础规则
  - Rust 中的每一个值都有一个 **所有者**（*owner*）；
  - 值在任一时刻有且只有一个所有者；
  - 当所有者（变量）离开作用域，这个值将被丢弃；
- 变量作用域
  - 作用域是使用{}包含的一段代码段；
  -  作用域是一个项（item）的有效作用范围；
  - 当 `s` **进入作用域** 时，它就是有效的；
  - 这一直持续到它 **离开作用域** 为止；
- 内存分配
  - 声明变量时，程序申请变量所需内存；
  - 内存在拥有它的变量离开作用域后就被自动释放；
  - 当变量离开作用域，Rust 为我们调用 drop 函数，清除特定内存；
  - 变量赋值给另一个变量时移动出现移动，当持有堆中数据值的变量离开作用域时通过 `drop` 被清理掉，除非数据被移动为另一个变量所有；
  - **移动**
    - **二次释放**：由于存储在堆上的数据，变量名存储的是指向内存的指针，如果赋值给一个新变量，在离开作用域时释放内存的时候会出现多次释放同一位置内存的问题；
    - **移动**：使用 = 将存储在堆上的对象赋值给新变量时会使得之前的变量失效；
    - 移动之后再使用之前变量会导致编译出错（borrow of moved value: `s`）；
    - 移动之后由于旧变量已无效，已无需释放内存，因此只会释放一次内存，避免了二次释放问题；
  - **克隆**
    - 使用 clone 是将堆中的数据复制一份，新变量存储着指向复制后数据的指针；
  - **拷贝**
    - 使用 = 将存储在栈上的数据赋值给新变量就直接将栈上数据复制一份给新变量；
    - 实现copy的类型就是使用拷贝；
    - 任何一组简单标量值的组合都可以实现 `Copy`；
    - 任何不需要分配内存或某种形式资源的类型都可以实现 `Copy`；
    - 实现了copy的类型
      - 所有整数类型，比如 `u32`；
      - 布尔类型，`bool`，它的值是 `true` 和 `false`；
      - 所有浮点数类型，比如 `f64`；
      - 字符类型，`char`；
      - 当且仅当其包含的类型都实现 `Copy` 的元组；
- 所有权与函数
  - 函数传参的规则也与=赋值规则一致；
  - 对未实现copy的类型采取的就是移动的方式；
  - 当出现移动的情况时，程序编译出错（borrow of moved value: `s`）提示你这个地方产生了移动，需要你使用clone的方式进行传参；
  - 实现copy的类型采取的就是拷贝的方式；
  - 函数返回值采取的是移动的方式返回给调用它的函数；

#### [4.2 引用](./scope/references_and_borrowing/src/main.rs)

- **基本知识**

  - 像一个指针，存储的是一个地址，能够访问储存于该地址的属于其他变量的数据；

  - 与指针不同在于引用指向的数据具有特定数据类型，不可更改；

  - 函数定义时采用 (&数据类型) 的方式表明需要传入引用，使用时则采用 (&数据名)方式表示传递指定数据的引用；

  - 引用实际上表示 可以使用该数据，但却不获取该数据的所有权；

  - 由于引用没有数据的所有权，因此在离开对应的作用域时，也不会丢弃和清理对应的数据内存；

  - 和变量默认不可变一样，引用也是默认不能被更改的；

- **可变引用**

  - 同时在传参除数据名前以及接收参数定义的类型名前添加(mut)关键字将使得我们能够在使用时修改引用对应的值；
  - 只有源数据值也是可变时，才能作为可变引用进行传参；
  -  在任意给定时刻，一个对象 **只能拥有一个可变引用或任意数量的不可变引用之一**；
  - **数据竞争**产生原因
    - 两个或更多指针同时访问同一数据；
    - 至少有一个指针被用来写入数据；
    - 没有同步数据访问的机制；
  
- **悬垂引用**

  - **悬垂指针**：指向的内存可能已经被分配给其它持有者；
  - 通过确保数据不会在其引用之前离开作用域来避免**悬垂引用**问题的发生；

#### [4.3 Slice 类型](./scope/slice/src/main.rs)

-  *slice* 允许引用**集合中一段连续的元素序列**，而不用引用整个集合；
- **字符串slice**
  - **字符串 slice**（*string slice*）是 `String` 中一部分值的引用，声明写做 &str；
  - &字符串变量名[startIndex..endIndex] 表示从该变量从startIndex到endIndex的局部引用；
  - 字符串 slice 引用包含startIndex，不包含endIndex；
  - 当 startIndex 为0时，可以省略，直接使用[..endIndex]表示从0到endIndex的局部引用；
  - 当endIndex 表示到 String 的结尾，endIndex 也可被省略；
  - 源数据出现任意改变时候会导致该数据绑定的 字符串slice 失效；
  - 字符串字面量数据类型实际上就是 字符串slice；
  - 对字符串的引用 相当于 对字符串的全量 slice；

### 5.结构体

#### [5.1 定义与实例化](./struct/define_and_instant/src/main.rs)

- 使用 **struct** 关键字定义一个结构体；
- 结构定义语法：struct 结构名 {结构体}；
- 结构体名采用 **首字母大写的驼峰命名** 方式进行命名；
- 结构体中 **字段** 采取 **字段名:字段类型** 的方式进行定义；
- 不同字段之间使用 逗号（,）进行分隔；
- 结构体实例化采取的是 **结构名 {key:value,key:value,...}** 方式进行；
- 实例化语句中的 key:value 表示 将名为 **key 字段赋值为 value**；
- 在结构体实例化之后，可以使用 **实例化对象名.字段名** 的方式访问对应字段数据；
- 在实例化对象可变时，能够使用 **实例化对象名.字段名** 的方式给对应字段赋值；
- 只能设置实例化对象是否可变，而不能单独设置某个字段是否可变；
- 若参数名与字段名一致，则可以省略 :参数名 直接使用字段名对指定字段赋值；
- 结构体更新语法
  - 若新的示例对象与旧对象部分字段值相同，可以使用 **结构名{差异字段赋值, ..旧实例名}** 的方式进行赋值；
  - ..旧实例名 只能放在**赋值语句的结尾**）；
  - .**.旧实例名** 相当于 **=** 符号赋值，也会出现移动等相关问题；
- 使用 **Struct 结构体名{数据类型, 数据类型,...}** 定义无字段名的**元组结构体**；
- 元组结构体对对应数据访问与元组一致；
- 使用 **Struct 结构体名** 定义一个没有任何字段的**类单元结构体**； 

#### [5.2 方法与关联函数](./struct/method/src/main.rs)

- 与函数一样，方法也是使用 **fn关键字** 进行定义；
- 第一个参数总是 **self** ，代表的是本函数所属结构体实例；
- 在 impl 块中，使用大写开头的 **Self** 代表相关的结构体数据类型；
- 实际定义时，需要使用 & 前缀 表示借用，与其他引用一致，当需要改变对象时，也需要加上mut；
- 方法的定义
  - 使用 **impl 结构体名{impl块}** 定义方法所属的结构体上下文；
  - 在方法结构体可以定义多个方法；
  - 使用 fn 方法名( &self , 其余参数){方法体} 定义方法；
  - 其余参数可以为0个；
  - 调用方法时，需传除 &self 以外的所有参数；
- **关联函数**
  - 所有在 impl 块中定义的函数；
  - 关联函数参数列表不以 self 开头，因此并非为方法；
  - 构造函数常使用 new 命名的， 返回一个结构体新实例的函数；
  - 使用 **结构体名::关联函数名** 调用指定结构体的关联函数；
- 每个结构体都允许拥有多个impl块；

### 6.枚举

#### [6.1 定义](./enums/define/src/main.rs)

- 枚举是用来列举 **同一数据所有可能值** 的数据结构；
- 使用 **enum 枚举名{值1, 值2,...}** 对枚举进行定义；
- 每个枚举值被称为枚举的**成员**；
- 与访问 关联函数 一致，使用 **枚举名::枚举值** 方式获取某个枚举值；
- 枚举值的类型全部为 该枚举；
- 每个成员可以绑定 **不同类型和数量** 的数据；
- 使用 **成员名(数据类型1,...)** 声明成员与数据绑定语法；
- **任意类型的数据**放入枚举成员中：例如字符串、数字类型或者结构体；
- 与 结构体 一致， 可以使用 impl 来为枚举定义方法群；

#### **[6.2 match 控制流](./enums/match_control/src/main.rs)**

- match 表示分支选择，与switch... case 使用方法一致；
- 使用 **match 枚举参数 {枚举值1 => {符合1时执行代码}, 枚举值2 => {符合2时执行代码}...}** 的语法使用；
- 代码只有一行时可以省略{}；
- 使用{}包裹执行代码时，可以省略每个分支之后的 , ；
- 如果执行代码为一个表达式时，**分支的返回值**将作为整个match控制流的返回值；
- 在匹配分支表达式枚举值可以使用 **枚举值(绑定数据名)** 作为条件以获取该枚举值绑定的数据；
- 匹配需要被穷尽的，若有**一个枚举值没被列举出来，将会报错**；
- 除了匹配枚举，还能匹配基础数据；
- 不想穷举所有选项时，可以使用 **other或者通配符(_)** 表示其余情况；
- 当只**需要一个分支时** 可以使用 **if let** 的方式替代match；

### 7.模块管理

#### 7.1 包和 crate 

- 使用rustc 命令编译时，编译器将编译的 rs 文件被视为一个 **crate**（类似于java中的class字节码文件）；
- crate 文件有二进制项和库两种类型
  - **二进制项** 是拥有main函数文件所编译的可执行文件；
  - **库** 没有main函数，只提供一些诸如函数之类的东西以供其他项目使用；
- **包（package）** 是提供一系列功能的一个或者多个 crate；
-  一个包会包含一个 *Cargo.toml* 文件用来阐述如何构建这些 crate ；
- 使用 cargo new 命令所创建的 **cargo 项目** 即是一个包；

#### **[7.2 模块](./module_system/model_test/src/main.rs)**

- **模块的声明**
  - 由于编译器是从包的 **根文件(二进制项为src/main.rs，库为src/lib.rs)** ，因此模块首先使用 **mod关键字** 在根文件中声明；
  - mod块声明的三种方式
    - **内联**：直接在声明语句之后加上使用 **大括号包裹的模块语句** 进行声明；
    - 声明存在与 **src/模块名.rs** 文件中的模块（声明语句使用分号结尾）；
    - 声明存在与 **src/模块名/mod.rs** 文件中的模块（声明语句使用分号结尾）；
  - **子模块**的声明
    - 在模块中同样也可以按照上述格式声明对应子模块；
    - 子模块路径为 **/父模块路径/父模块名/对应子模块路径**；
- **路径**
  - **路径格式**：使用 **::** 对路径文件与文件夹进行分隔，完整格式 **前缀::模块名::引用代码块名（函数名、结构体名）** 定义；
  - **绝对路径**：外部 crate 的代码，是以 crate 名开头的绝对路径，当前 crate 的代码，则以字面值 `crate` 开头；
  - **相对路径**：从当前模块开始，以 、`super` 或当前模块的标识符开头；
  - **super**：从父模块开始构建相对路径，而不是从当前模块或者 crate 根开始（与文件系统中的 .. 符号一致）；
  - **self与当前模块标识符**：从当前模块模块开始构建相对路径；
- 模块权限控制
  - **默认所有项（函数、方法、结构体、枚举、模块和常量）**只对其父模块可见，只用在声明时加上 **pub关键字** 才能使之公用；
  - **pub 修饰的结构体**会变成公有的，但其中字段仍然是私有的。根据情况决定每个字段是否公有；
  - **pub 修饰的枚举** 所有的数据都是共有的可以访问；
- 模块使用
  - 使用 **use 模块路径** 的方式可以将指定模块引入到本 crate 作用域中；
  - 使用时直接使用 **模块名::引用代码块名** 直接使用；
  - 可以进一步通过 **use 模块路径::引用代码块名** 直接引用指定代码块，使用时直接使用 **引用代码块名** 进行调用；
  - use 语句引入的模块作用 只作用于当前所在作用域，其子模块作用域无法使用；
  - **习惯用法**
    - 引入 **函数** 时一般使用 **use 模块路径** 进行引入；
    - 引入除函数以外的其他项（**结构体、枚举等** ）则是直接使用 **use 模块路径::引用代码块名** 的方式；
  - 使用 **use 语句 as 别名** 的方式可以指定引用对象的别名；
  - **重导出**
    - 在 use 语句之前加上 **pub 关键字**，可以将一个名称导入了当前作用域同时还允许别人把它导入他们自己的作用域；
    - 可以在引入当前工作域的文件，直接使用 **当前工作域名 引入模块使用** 直接使用对应模块或模块中的代码；
  - 使用cargo 引入的外部包时，使用 **包名::引入对象** 的方式引入指定对象；
  - 在引用的对象具有通用前缀时，使用 **use 通用前缀::{对象1,对象2...} 嵌套路径** 的方式合并多条 use 语句；
  - 使用  *** ，glob 运算符** 可以将一个路径下 **所有** 公有项引入作用域；

### 8. 集合

#### [8.1 vector](./collection/vector/src/main.rs)

- vector 中的数据使用 **Option** 枚举进行包装；
- 新建
  - vector 对象的结构体名为 **Vec** ；
  - 创建时使用 Vec 中的 **new()** 关联函数进行创建；
  - 由于 Vec 需要确定其中元素的数据类型，因此需要通过 **Vec<数据类型>** 的方式声明元素数据类型；
  - 使用 **vec!宏** 可以创建对应值的 Vector 对象；
  - 在使用 **vec!宏** 时，程序可以通过值推断类型；
- 操作 vector 元素
  - 当后续需要对 vector 对象进行修改时，定义时使用 **mut** 进行修饰；
  - 使用 **remove(index)** 方法移除 vector 对象中index对应数据；
  - 使用 **push(数据)** 方法向 vector 对象尾部添加数据；
  - 使用 **pop()** 方法弹出 vector 对象；
  - 使用 **get(index)** 获取指定index中的数据；
  - 由于 vector 中的元素使用 Option 枚举进行包装，因此当get 方法中index 超出界限，数据值为 **Option::None**；
  - 除了使用 **get** 方法获取对应数据，还可以使用 **&v[index]** 的方式直接获取对象数据的引用，而**不是Option包装**数据；
  - 使用 **&v[index]** 方式时index 超出界限时导致报错；
  - 使用 **&v[index]** 方式其实是借用整个 vec 对象，任何 **vector的修改** 将会导致 引用失效；
  - 使用 **for 元素名 in &对象名** 遍历整个对象；
  - 在丢弃 vector 对象时， 所有其内容也会被丢弃 ；

#### [8.2 String](./collection/string/src/main.rs)

- String 是一个对 **Vec<u8>** （元素为 utf-8 的 Vec 结构体）的封装；
- 由于字符串索引应该返回的类型是不明确的，可以为**字节值**、**字符**、**字形簇**或**字符串 slice**， 因此 Rust 不允许使用索引获取 `String` 字符；
- 新建
  - String 对象的结构体名为 **String** ；
  - 使用 String 中的 **new()** 关联函数进行创建**空字符串**；
  - 使用 String 中的 **from(字符串字面量)** 关联函数进行创建 **指定字面值** 的字符串；
  - 使用 **字符串字面量.to_string()** 也能创建 **指定字面值** 的字符串；
- 修改
  - 使用 **push_str(字符串字面量)** 方法将 字符串字面量 添加到对象后面；
  - 使用 **push(字符)** 方法将字符拼接至 字符串 之后；
  - 使用 **字符串名 + &字符串名** 拼接字符串；
  - 在使用 **+ 运算符** 之后，第一个字符串变量由于移动将会失效；
  - 使用 **format! 宏** 可以根据需求拼接多个字符串；
  - **format! 宏** 中宏与 **println! 宏** 的格式一致；
- 字符串的遍历
  - 字符串遍历之前之前需要使用 **chars()**、**bytes()** 等方法获取对应数组，然后对数组进行遍历以获取对应参数；

#### [8.3 map](./collection/map/src/main.rs)

- HashMap 默认使用一种叫做 **SipHash** 的哈希函数；
- 新建
  - 由于HashMap没有被 prelude 自动引用，因此在使用时需要通过 **use std::collections::HashMap; ** 引入 hashMap 对应结构体；
  - 使用 HashMap 中的 **new()** 关联函数进行创建 **空Map**；
  - 在创建 HashMap 时，可以通过声明变量类型为 **HashMap<key数据类型，value数据类型>** 显示声明对应数据类型；
  - 在创建 HashMap 时，未声明数据 key 与 value 数据类型时，程序通过 **第一次往 HashMap 对象里插入的数据** 推断其数据类型；
- 操作map对象
  - 使用 **get(key)** 方法获取使用 **Option 枚举包装的value对象**；
  - 当 key 不存在时，获取的值为 **Option::None**;
  - 使用 **for (key, value) in HashMap对象** 来遍历指定hashMap 对象中 key，value 值；
  - 使用 **insert(key, value)** 向map中添加参数；
  - 在 HashMap 对象中，使用 **insert(已存在指定 key 值, 新value)** 时将会 **替换** 指定 key值对应的value；
  - 使用 **entry(key).or_insert(value)** 方法，先判断 map 对象中是否存在 key ，如果不存在的话插入 (key, value)；
  - **entry(key)** 方法的返回值是一个 **Entry 枚举**，代表了可能存在也可能不存在的值；
  - Entry 枚举 的 **or_insert(value) 方法** 在键对应的值存在时就返回这个值的可变引用，不存在则将 value 作为新值插入并返回新值的可变引用；
  - 由于 **entry(key).or_insert(value)** 返回值为一个可变引用，因此修改该引用会导致该value值的改变；

### 9.错误处理

#### [9.1 不可恢复错误的处理](./error_handling/unrecoverable_error_handler/src/main.rs)

- **不可恢复的错误**： bug 出现的征兆，比如试图访问一个超过数组末端的位置，因此需要立即停止程序；
- 使用 **panic! 宏** 方式在出现的不可恢复错误时，打印错误信息，展开并清理栈数据；
- 出现 panic 的两种方式
  - 在代码执行过程中，由于程序出现不可恢复错误造成 **程序自动调用 panic** ；
  - 使用 **panic! 宏** 手动调用执行；
- 错误处理指导原则
  - 出现了会破坏系统正常运行的情况下直接使用 panic！
    - 一些假设、保证、协议或不可变性被破坏的状态；
    - 与偶尔会发生的行为相对的 **非预期的行为**；
    - 之后的代码运行 **基于该错误的状态**；
    - 没有方法将错误状态信息编码进所使用的类型中；

#### [9.2 可恢复错误的处理](./error_handling/recoverable_error_handler/src/main.rs)

- **可恢复的错误**：比如文件未找到的错误，可以通过重试或其他操作使得程序能继续运行；

- **Result<T,E> 枚举** 作为一个函数返回执行是否对象

  -  **OK(T)数据** 表示执行成功时的返回值（T表示执行成功结果）；
  -  **Err(E)数据** 表示执行错误时的返回值（E表示错误类型）；
  -  **unwrap方法** 执行成功返回对象，失败时则是执行对应的panic；
  -  **expect(message)方法** 与 unwrap方法逻辑一致，可以设置对应panic的message；

  -  使用 match 对执行结果进行处理；
  -  使用 **E的kind()方法** 获取 ErrorKind 枚举；

-  **错误的传播**

   -  当在本方法中不想处理对应错误时，可以将错误作为返回值，交给调用者进行处理；
   -  使用 **? 运算符 结尾 ** 表示若该语句出现了错误直接向上传播该错误；
   -  **?运算符** 只能被用于返回值与 `?` 作用的值(**实现了 FromResidual 类型**)相兼容的函数；
   -  **?运算符** 也可用于提前返回 **Option 枚举** 中的None相关值；

### 10.泛型和生命周期

#### [10.1 泛型](./generic_and_lifecycle/generic/src/main.rs)

- 使用代表多种类型的 **占位符** 代替具体类型减少代码冗余的方式；
- 定义了多个泛型时，使用对应数据必须 **声明全部泛型** 的具体类型；
- 编译时进行泛型代码的 **单态化** 来保证泛型效率，即是在编辑时填充运行时的具体类型将代码转化为特定代码；
- 在 **函数定义** 中使用泛型
  - 在函数名之后使用 **<占位符>** 定义泛型类函数；
  - 在指定函数参数和返回值类型的地方使用泛型代替；
  - 在调用函数时，可以传递符合指定泛型的不同类型的参数；
- **结构体** 中定义泛型
  - 与函数中定义泛型的方法一致，在结构名后使用 **<占位符>** 定义泛型名；
  - 在结构体实例化时，可以是使用 **结构体名::<具体类型>** 显示指明泛型的具体类型；
  - 也可以与普通结构体声明一致，rust通过代码推测泛型实际类型；
- 枚举声明泛型的方式与结构体声明泛型的方式一致；
- 在 impl 块中使用泛型
  - 使用 **impl<泛型> Point<泛型>** 声明impl块中使用的泛型；
  - 在 impl 块中的方法与关联函数中直接使用泛型；

#### [10.2 trait](./generic_and_lifecycle/trait_code/src/main.rs)

- 以一种 **抽象的方式** 定义共享行为的方法（类似于其他语言中的**接口**）；
- 定义
  - 使用 **trait** 关键字进行定义；
  - 定义在 trait 中方法 **可以没有方法体** ，表明该方法需要在后续继承trait的结构体中实现；
  - 也可以定义一个具有 **默认实现** 的方法；
- 实现
  - 使用 **impl trait名 for 结构体名** 为指定结构实现对应trait方法；
  - 在实现的代码块中，**必须实现trait中没有方法体的抽象方法**；
  - **trait 或要实现的结构至少有一个位于本地作用域** 时，才能为该结构实现 trait；
  -  **相干性** ：为了确保其他人编写的代码不会破坏你代码，因此不能为外部类型实现外部 trait；
- 使用
  - 使用 trait 作为参数
    - 使用 **&impl trait名** 表明接收的参数需要实现指定 trait；
    - 使用 **&(impl trait名1 + trait名2...) ** 表明指定参数需要实现多个trait；
    - Trait Bound 语法
      - 在声明泛型时，在泛型名后加上 **:trait名** 表明该泛型参数需要实现指定trait；
      - 在泛型名后加上 **:trait名1 + trait名2 ...** 表明该泛型需要同时实现多个trait；
    - 在函数声明的返回值后使用 **where 泛型名1:trait名1 + trait名2 ... , 泛型名2:trait名1 + trait名2 ...** 可以清晰化trait bound 语法；
  - 将 **impl trait名** 作为返回值类型时，表明函数的返回值需要实现指定trait；

#### [10.3 生命周期](./generic_and_lifecycle/lifecycle/src/main.rs)

- 生命周期 指的是引用 **保持有效** 的作用域；
- rust 使用 **借用检查器** 来比较作用域，进而确保所有的借用都是有效的；
- 生命周期注解：
  - 注解语法：以 **撇号(')** 开头，名称通常全是小写；
  - 使用方式
    - **&'泛型生命周期名 数据类型** 表明指定数据类型的生命周期引用；
    - **&'泛型生命周期名 mut 数据类型** 表明指定数据类型的可变生命周期引用；
  - 函数中使用
    - 语法如申明泛型一致，现在尖括号中使用 **'生命周期名** 声明生命周期，然后在参数列表与返回值对应位置进行声明参数是否带生命周期；
    - 使用生命周期注解修饰多个参数表示该生命周期注解 **获取带该注解传入参数生命周期的交集** ；
    - 使用生命周期注解修饰的返回值的生命周期与 **该泛型的生命周期一致**；
    - 生命周期注解只存在于 **函数签名** 之中；
  - 结构体中使用
    - 语法与给结构体声明泛型的方式一致；
    - 使用生命周期注解修饰多个字段表示生命周期注解 **获取带该注解字段的生命周期的交集** ；
    - **结构体的生命周期 **与生命周期注解一致；
  - 方法中使用生命周期，与impl块中声明一致，表明指定返回值与结构体生命周期一致；
- 函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配；

- 生命周期注解的省略规则
  - 函数或方法的参数的生命周期被称为 **输入生命周期；**
  - 返回值的生命周期被称为 **输出生命周期**；
  - 编译器为每一个引用参数都**默认会分配一个生命周期参数**；
  - 只有 **一个输入生命周期参数** 情况下，他的生命周期参数将会被自动赋予所有输出生命周期参数；
  - 方法存在多个输入生命周期参数但有一个 **self** 参数，则所有输出生命周期参数被赋予 **self 的生命周期**；
- 静态生命周期
  - 语法 **'static 数据类型** 表明该数据生命周期可存在与整个程序存活期间；
  - 所有 **字符串字面量** 都是使用静态生命周期修饰的；

### 11.自动化测试

#### [11.1 编写测试](./auto_test/writing_tests/src/main.rs)

- 使用 **#[test]** 注解对函数注解表明该函数是一个测试函数；
- 使用 **assert! 宏** 检查测试结果；
  - 使用时需要测试函数为宏提供一个结果为 **bool 类型的表达式** 作为参数；
  - 参数执行结果为 **true** 时，assert! 宏 不会产生任何操作，同时测试通过；
  - 参数执行结果为 **false** 时，assert! 宏调用 `panic!` 宏，导致测试失败；
- 使用 **assert_eq! 宏** 与 **assert_ne! 宏** 测试是否相等
  - 使用时需要测试函数为宏提供 **两个** 可以使用 **==进行对比的表达式** 作为参数；
  - 两表达式相等时，assert_eq! 宏不会产生任何操作，同时测试通过；
  - 两表达式不相等时，assert_eq! 宏调用 `panic!` 宏 **打印出等式两端的数据**，同时导致测试失败；
  - assert_ne! 宏 操作与 assert_eq! 宏 相反；
- 自定义失败信息
  - 可以向断言语句后面传递自定义参数，以定制测试失败后提示信息；
  - 在必要参数后部加入参数的形式，采用如 **format!宏 中拼接字符串** 参数的格式；
- 使用 **should_panic** 检查panic
  - 使用 **#[should_panic]** 注解测试指定代码是否按照需求进行panic；
  - 该注解修饰的测试方法 **进行 panic ** 时，表明测试成功；
  - 在需要进一步规范 panic 具体内容时，可以通过 **设置 expected 参数** 来检查 panic 出的 message 是否包含指定文本；
  - 该注解位于 **#[test] 之后**；
- 使用 **#[ignore]** 注解表示运行test命令时不运行该测试；

#### [11.2 测试结构](./auto_test/test_struct/src/lib.rs)

- 单元测试
  - 目的是在 **与其他部分隔离的环境** 中测试每一个单元的代码；
  - 单元测试与被测试代码 **共同存放在位于 *src* 目录下相同的文件中**；
  - 每个文件中创建 **使用 cfg(test) 标注 的 包含测试函数的 `tests` 模块**；
  - 只有在使用 cargo test 相关命令的条件下才会编译测试模块代码；
  - 单元测试模块中可以 **直接对私有函数** 进行测试；
- 集成测试
  - 集成测试则是用于测试 **外部库** 是否正常工作；
  - 集成测试的代码集中在与src目录同级下的 **tests 目录** 下；
  - 由于测试代码不属于该包，因此在使用前需使用 **use 包名** 引用指定模块；
  - 集成测试中使用子模块与正常使用一致；

### 12.命令行程序

#### [12.1 命令行参数的接收](./command-line-project/command-line-args-accept/src/main.rs)

- **std::env::args()**  函数
  - 函数返回一个传递给程序的命令行参数的 **迭代器**（*iterator*）；
  - 使用迭代器的 **collect()**  方法将其转化为一个 **vector 集合**；
  - 参数集合第一个元素为 **所运行的二进制文件名** ；
- 使用 **std::fs::read_to_string(文件路径字符串)**  函数读取文件，结果为 **Result枚举** ，未报错结果为文件内容的字符串格式；

#### [12.2 程序结构优化](./command-line-project/upgrade/src)

- 二进制文件功能分离
  - 将程序拆分成 *main.rs* 和 *lib.rs* 并将 **程序的逻辑放入 lib.rs** 中；
  - 当命令行 **解析逻辑比较小** 时，可以保留在 *main.rs* 中；
  - 当命令行 **解析开始变得复杂** 时，也同样将其从 *main.rs* 提取到 *lib.rs* 中；
  - main.rs 保留的功能
    - 使用参数值 **调用命令行解析逻辑**；
    - 设置 **其他的配置**；
    - 调用 **lib.rs 中的 run 函数**；
    - 处理 **run 返回错误**；
- 参数解析器的提取
  - 将 **参数处理逻辑** 提取到一个函数中去；
  - 验证参数数量；
  - 指定对应参数功能；
  - 可以使用一个结构体接收所有参数，并指定用途；
- 程序逻辑的提取
  - 与参数解析器一样，在程序发生错误时也需要将错误返回给主函数进行处理；
- 使用 **eprintln!宏** 将信息记录到标准错误文件中；

### 13.函数式语言功能

#### [13.1 闭包](./functional-features/closures/src/main.rs)

- 闭包是一种可以保存在一个变量中或作为参数传递给其他函数的 **匿名函数**；
- 闭包捕获被定义时 **所在作用域中的值**；
- 语法使用
  - **|传递参数| 函数体** 语法使用闭包；
  - 闭包传参时可以不必明确指定 **参数与返回值类型**，系统可以有效推断其类型；
  - 若函数体只有一行代码时，也可直接省略外部花括号；
  - 若闭包赋值的是一个变量，与函数使用方式一致，使用 **变量名(传参列表)** 方式进行调用；
  - 与其他可以通过系统推荐数据类型的用法一致，在 **第一次传递参数时会对指定参数数据类型** 进行锁定；
- 捕获引用
  - 对于不可变引用，在声明闭包与使用闭包之间 **可以使用被引用对象**；
  - 对于可变引用，在声明闭包与 **最后一次使用** 闭包之间 **不允许使用被引用对象**；
  - 可变引用是在 **定义闭包即获取引用所有权**，直到之后一次使用闭包；
  - 在定义闭包前使用 **move** 关键字可以使闭包强行获得对应引用对象所有权，导致环境变量失效；
- Fn trait
  - FnOnce 表示闭包 **只能能被调用一次**，所有闭包都至少实现了这个 trait，一个会将捕获的值移出闭包体的闭包只实现 `FnOnce` trait，只能被调用一次；
  - FnMut  表示 **不会将捕获的值移出闭包体且可能会修改被捕获的值的闭包**；
  - Fn 表示闭包 **既不会将被捕获的值移出闭包体也不会修改被捕获的值** (包括不从环境中捕获值)；

#### [13.2 迭代器](./functional-features/iterators/src/main.rs)

- 迭代器的创建
  - 迭代器是 **惰性的**（*lazy*），在使用迭代器之前，迭代器都不会获取集合的所有权，只有在使用之后才会获取集合所有权；
  - 调用  **集合对象中的 iter() 方法** 创建一个以该对象为基础的迭代器；
  - 使用 **for 元素名 in 迭代器名 循环体** 循环遍历指定集合对象；
  - 消费迭代器的方法
    - 消费迭代器的方法会 **获取迭代器的所有权**，因此该类型方法在使用之后 **迭代器就失效** 了；
    - **next() 方法**  将迭代器中记录序列位置的向后移动一格 （返回值为**包含指向记录位置的 Option 枚举值**）；
    - **sum() 方法** 获取迭代器中所有元素和；
  - 使用 **迭代器适配器** 可以将当前类型的迭代器转化为不同类型的迭代器；
  - 链式调用多个迭代器适配器来以一种可读的方式进行复杂的操作；
  - 调用一个消费适配器方法从迭代器适配器调用中获取结果；
  - 很多迭代适配器接受闭包作为参数；

### 14.智能指针

#### [14.1  Box<T>](./smart_pointer/box_pointer/src/main.rs)

- 使用Box的构造函数 **new(数据)** 在堆上创建指定数据，创建指向堆上数据的指针；
- 通过访问 Box 对象 **直接访问存储的数据**；

- 在离开作用域时，不仅会 **释放 Box 指针对象，还会释放其所指向的堆上数据**；
- 递归类型
  - 表示该类型数据 **拥有另一个同一类型数据**；
  - 由于递归类型没有已知大小，因此 **不能直接使用**；
  - 递归类型 **只能使用 Box 指针** 进行创建；
- 使用 **解引用运算符（*）** 可以获取引用对应数据；
- 与引用一致，**解引用运算符（*）** 也可以获取到智能指针封装的原值；
- 自定义指针
  - 自定义指针需要 **实现 Deref trait**；
  - 在实现体中，定义  **type Target = 泛型名** 同时实现  **deref(&self) -> &Self::Target**  方法；
  - 在使用解引用符修饰自定义指针时，底层调用 *** (自定义指针对象.deref()) ** 对自定义指针解引用；
- 隐式 Deref 强制转换
  - **一种只能作用于实现了 `Deref` trait 的类型** 的函数或方法传参上的便利操作；
  - 在将特定类型的引用作为实参 **传递给形参类型不同的函数或方法时** 自动进行；
  - 强制类型转换条件
    - 当 T: Deref <Target=U>  时从 &T 到 &U；
    - 当 T: DerefMut <Target=U>  时从 &mut T 到 &mut U；
    - 当 T: Deref <Target=U>  时从 &mut T 到 &U；
- Drop trait
  - 实现 Drop trait 的结构体 **需要实现 drop 方法**；
  - 实现 Drop trait 的实例化对象在其离开作用域销毁时，系统 **自动调用 drop 方法**；
  - drop 方法 **不能手动进行调用**，来提前清理对应实例化对象；
  - 可以使用 **std::mem::drop(清理对象名)** 函数提前回收指定对象内存；
  - **手动清理对象内存** 时，系统也会自动调用对象的 drop 方法；

#### [14.2 其他指针](./smart_pointer/other_pointer/src/main.rs)

- 引用计数只能指针（std::rc::Rc <T>）
  - 该指针可以显示启用多所有权；
  - 使用 **Rc::clone(指定对象引用)**  对指定对象增加引用计数值，从而使新 Rc 指针指向指定对象；
  - 使用 **Rc::strong_count(指定对象引用)** 获取指定对象当前引用计数器值；
  - Rc 指针 **只能指向不变引用**；
- RefCell<T> 指针
  - 内部可变性：Rust 中的一个设计模式，允许 **对象在有不可变引用的情况下也可改变其数据**；
  - RefCell 允许在 **运行时执行可变借用检查**，使得即便 **RefCell 自身是不可变的情况下修改其内部的值**；
  - RefCell 也只能是 **单一所有权拥有者**；
  - RefCell 对象的 **borrow_mut()** 方法可以获取指针指向对象的可变值；

### 15.并发编程

#### [15.1 多线程](./concurrent_programming/threads/src/main.rs)

- 多线程执行问题
  - **竞态条件**：多个线程以不一致的顺序访问数据或资源；
  - **死锁**：两个线程相互等待对方，这会阻止两者继续运行；
- 使用  **std::thread:: spawn(无参数闭包)** 创建一个新线程；
- 无参数闭包中的 **函数体** 即新线程运行代码；
- spawn 函数的 **返回新增的线程对象**；
- 使用 **thread::sleep(Duration::from_millis(1))** 函数使函数强制所在线程停止执行一小段时间；
- 使用线程对象的join方法表示，当前线程需 **等待指定线程对象运行完后才会继续执行**；
- 新建线程中需要使用外部环境变量时，需 **使用 move 关键字将环境变量所有权转移到新线程之中**；

#### [15.2 消息传递](./concurrent_programming/message_passing/src/main.rs)

- rust 使用 **信道** 将数据从一个线程发送到另一个线程；
- 信道中存在 **一个 发送者与一个接收者**；
- 当发送者或接收者任一被丢弃时可以认为信道被 **关闭** 了；
- 信道定义
  - 使用 **std::sync::mpsc ::channel() ** 创建信道，返回值为一个 **(生产者,消费者)** 元组；
  - 一个信道可以有 **多个生产者**，但只能有 **一个消费者**；
  - 通过调用 **生产者.clone()** 方法复制发送者以产生多个生产者；
  - 使用 **生产者.send(消息)** 发送指定消息；
  - 使用 **消费者.rec()** 消费消息；
  - 当信道中存在多个消息时，使用 **for 消息 in 消费者对象** 消费多个消息；

#### [15.3 线程共享（Mutex）](./concurrent_programming/shared/src/main.rs)

- **互斥器**：同一时刻只允许一个线程访问数据；
- 为了访问互斥器中的数据，需先 **获取到互斥器的锁**；
- 互斥器中数据使用完之后，需要 **释放互斥器的锁**；
- Mutex<T> api
  - 使用 **构造函数 new(互斥器数据)** 创建封装指定数据的互斥器对象；
  - 使用互斥器对象中的 **lock()** 方法在获取互斥器对象封装的数据与锁；
  - 如果在第一时间未获取到锁，阻塞当前线程直到能获取到；
  - 如果某个线程 **拥有当前锁的同时报错了**，这时其他线程使用lock方法获取该锁时会报错；
  - 获取到的锁，在 **离开当前作用域时自动释放**；
  - 与 RefCell 一样， **Mutex 也提供内部可变性**；
- 线程间共享
  - 使用 **原子引用计数(std::sync::Arc<T>)** 将互斥器传递给不同的线程；
  - 原子应用计数与引用计数api一致，使用 clone 方法增加原子应勇计数；

### 16.面向对象编程

#### 16.1 面对对象特点

- 对象
  - 程序 **由对象组成**；
  - 对象由 **数据与操作这些数据的过程**（过程称之为方法或者操作）组成；
  - rust 中 **结构体和枚举** 包含数据；
  - **impl 实现块** 中实现了操作相关数据的方法；
- 封装
  - 调用对象时 **不需要知道对象中的实现细节**；
  - rust 中 **是否使用 pub 关键字** 来屏蔽对象封装细节；
- 继承
  - 通过继承可以 **获得父对象的数据和行为**，无需重新定义；
  - rust 没有直接的继承相关语法；
- 多态
  - 多种对象共享特定的属性，可以相互替代使用；

#### [16.2 trait对象](./oop/trait_object/src/main.rs)

- 语法
  - **dyn trait约束组** 声明的trait对象，表示该处需要的类型为需要实现指定 trait 的类型；
  - 除首个 trait 外，trait约束组中 **其他所有 trait 都必须是自动trait**；
  - trait约束组中 **生存期 trait 不能超过一个**；
  - 约束组中 **不允许选择退出约束**；
- 与泛型指定需要实现类型不同，泛型类型需要在声明结构时确定，当 dyn 却 **能在运行时使用多种类型**；
- 鸭子类型：只关心 **值所反映的信息** 而不是其具体类型；
- 动态分发：编译器生成的代码只有在 **运行时才能确定调用的具体方法**；
- 静态分发：编译器构建代码时，**声明的泛型类型会被替换为具体类型** 而生成非泛型代码实现； 
- 使用 trait 对象时，编译器采用的是 **动态分发** 的方式进行实现的；
- 类型安全
  - 安全规则
    - 定义方法的 **返回值不是 Self**（使得编译器能确定返回值具体类型）；
    - 方法中 **没有泛型类型的参数**；

### 17.模式

#### [17.1 模式的匹配](./pattern/pattern_match/src/main.rs)

- **模式**是用来匹配类型中的结构，无论类型是简单还是复杂的语法；

- 模式匹配使用有效位置
  - match分支中使用模式对比值与不同分支；
  - if let 表达式进行 **单次模式匹配**， 匹配成功将等号右边值赋值给左边，同时执行后续代码块；
  - if let 可以 **与 else if、else 以及 else if let 表达式联合使用**；
  - while let 则是 **循环对指定等于表达式进行模式匹配**，直到出现不匹配情况；
  - for 循环与while let 一致，**循环对指定等于表达式进行模式匹配**，不做判断，模式匹配失败会panic；
  - **let 语句与函数参数传递** 也暗含了模式匹配；
- 可反驳性
  - 不可反驳：**任何传递的可能值必须与指定模式相匹配** 的模式；
  - 可反驳：**某些可能的值匹配可能失败** 的模式；
  - **函数参数、let` 语句和 `for 循环** 只能接受不可反驳的模式；
  - 带判断的相关语句，匹配模式则一定是可反驳的；

#### [17.2 匹配语法](./pattern/match_grammar/src/main.rs)

- **直接匹配字面量值**：直接将字面量值匹配并赋值给变量，通常let 声明变量即是使用该模式；
- **命名变量的匹配**：在可反驳模式下，一旦模式匹配成功，会将对应值赋值给对应变量，使用 match 匹配枚举时经常会使用；
- 在可反驳模式下，使用  **| 运算符（代表或）** 可以匹配多个模式；
- 在可反驳模式下，**起始值  ..=  终点值** 表示可以匹配 从起始点到终点值范围内的值（可以是数字、字符等范围）；
- 解构并分解对象
  - 解构解构体
    - 使用 **let 结构体名{字段名1:变量名1, 字段名2:变量名2...} = 结构体对象** 对指定结构体实例化对象进行解构，将对象对应值赋给指定变量；
    - 字段名:变量名 可以直接简写为 **字段名** 表示变量名与字段名一致；
  - 解构枚举
    - 与解构结构体一致，枚举也可以通过匹配模式 **解构枚举中包含的具体值**；
  - 可以解构结构体、枚举、元组等数据的嵌套；
- 忽略模式中的值
  - _符号
    - 使用 _ 忽略整个值：表明 _ 所在位置忽略指定位置参数，函数中不会使用该值；
    - 可以对 **枚举或元组** 中的某个值使用 _ 表示指定参数不会被使用；
    - 也可以在声明变量时，以 **_ 开头命名变量** 表明该变量不会被使用；
  - .. 符号
    - 使用 **.. 符号** 可以忽略除了指定数据外的所有其他元素；
    - **第一个元素, ..** 只解构第一个元素数据；
    - **第一个元素, ..，最后一个元素** 解构第一个元素与最后一个元素数据；
    - **..，最后一个元素** 只解构最后一个元素数据；
- 匹配守卫
  - **匹配守卫** 用于指定 `match` 分支模式之后的额外 `if` 条件，表明该分支 **必须满足match匹配与 if 条件**；
-  **@运算符** 允许 **创建一个存放值的变量** 的同时测试其值是否匹配模式；

### 18.高级特性

#### [18.1 不安全rust](./advanced_features/unsafe_rust/src/main.rs)

- 使用 **unsafe 关键字** 将rust切换到不安全状态；
- 不安全模式只保证 **解引用裸指针、调用不安全的函数与方法、访问或修改可变静态变量、实现不安全trait及访问union 的字段** 五个功能不会被编译器检查内存安全；
- 解引用裸指针
  - **&变量 as *const 引用类型** 声明指向指定变量的不可变裸指针;  
  - **&mut 变量 as *mut引用类型** 声明指向指定变量的可变裸指针;  
  - as 关键字将引用 **强转** 为指定裸指针；
  - **安全代码** 中可以创建裸指针，但只能 **在不安全块之内解引用裸指针**；
  - 与引用的区别
    - 允许忽略借用规则，可以**同时拥有不可变和可变的指针**，或多个指向相同位置的可变指针；
    - 不保证指向有效的内存；
    - 允许为空；
    - 不能实现任何自动清理功能；
- 调用不安全的函数与方法
  - 使用 **unsafe 关键字修饰的函数与方法** 就是不安全的函数或者方法；
  - **不安全的函数的函数体** 本身也是不安全块；
  - **只能在不安全的代码块中调用** 不安全函数与方法；
  - 使用 **extern 关键字** 调用外部代码；
    - 使用 extern 声明需要调用的外部代码块；
    - 将 **需调用的外部库函数** 放置在外部块中；
    - 调用外部函数也需要在 unsafe 代码块中调用；
- 访问修改可变静态变量
  - 通过使用 **static 关键字** 可以创建静态变量；
  - **静态变量是全局的** ，声明在函数之外；
  - 静态变量可以用 **mut 关键字** 修饰使之变成可变的；
  - 由于数据竞争相关问题，因此访问和修改可变静态变量时不安全的，需要在 **不安全块中** 进行访问；
- 对不安全的 trait 进行实现
  -  trait 存在 **任意一个方法中包含编译器无法验证的不变式时** 是不安全的， 该 trait 需要使用 unsafe 关键字进行修饰变成不安全 trait。
  - 实现不安全 trait 时，也需要使用 **unsafe 修饰 impl 块**；
- 访问联合体（union）中的字段
  - 使用 **union 联合体名 {联合体参数}** 的形式声明联合体；
  - 与结构体不同在于，结构体所有字段在不同内存，修改其中任意一个不会对其余字段产生影响，联合体中所有字段共享内存，**修改其中任意字段会影响剩余所有字段**；
  - 联合体主要用于与 **c语言中联合体** 进行交互；
  - 由于 联合体中各种字段是互相影响的，因此访问是不安全的，需要在不安全块中进行访问；

#### [18.2 高级trait](./advanced_features/advanced_trait/src/main.rs)

- 关联类型
  - 通过 **将类型占位符与 trait 相关联的方式** 以方便方法签名中使用指定类型；
  - 使用 **type 关联类型名** 的方式声明关联类型；
  - 在实现拥有 **关联类型 trait** 时，需要显示指定trait的具体类型；
  - 关联类型在 **实现 trait 时就会确定具体类型**，一个结构体的实现只能有一个具体的关联类型；
- trait 中声明泛型时可以指定默认参数类型，实现时 **不声明泛型时使用默认类型**；
- **运算符重载**：在特定情况下自定义运算符（比如 `+`）行为的操作；
- 默认参数类型使用：
  - 扩展类型而不破坏现有代码；
  - 在大部分用户都不需要的特定情况进行自定义；
- 完全限定语法
  - 在实现的多个 trait 中拥有相同方法或函数时，**默认调用直接实现在类型上的方法或函数**；
  - 使用 **trait 名::方法名(对象引用)** 的方式调用实现对应trait的方法；
  - 使用 **结构体名 as trait名::关联函数(参数)** 完全限定语法调用实现对应trait的函数；
- 定义 trait 时，在签名后使用 **: 父 trait** 的语法指定 trait 的父 trait；
- 在实现拥有 父 trait 的trait时，必须 **将其 父 trait 中的方法与函数一起实现**；

#### [18.3 高级类型](./advanced_features/advanced_types/src/main.rs)

- **newtype 模式**
  - 使用一个希望实现 trait 的类型字段作为简单封装的元组结构体实现；
  - newtype 模式可以用于 **抽象掉一些类型的实现细节**；
  - newtype 模式可以 **隐藏其内部的泛型类型**；
- 类型别名
  - 使用 **type 关键字** 来给予现有类型另一个名字（别名）；
  - 类型别名 与 类型原名是 **同义词**；
  - 原类型与类型别名可以协同计算，也可以将 类型别名 传递给 原类型函数参数；
  - 类型别名  