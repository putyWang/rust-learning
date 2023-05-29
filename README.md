## 从零开始学习Rust

### 1.配置rust开发环境

#### （1）windows安装

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

### 2.项目的创建



