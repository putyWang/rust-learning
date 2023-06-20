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

### 2.项目的创建

#### 2.1 命令： cargo new 项目名

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

#### [3.1 变量](/blob/main/hello-world/main.rs)

- 

