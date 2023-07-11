// 方式1声明 sub_test1 子模块
mod sub_test1{}

// 方式2声明 sub_test2 子模块(路径为/src/test1/sub_test2.rs)
mod sub_test2;

// 方式3声明 sub_test3 子模块(路径为/src/test1/sub_test3/mod.rs)
// pub 关键字表明 sub_test3 并非时 test1.rs 私有，而是公用模块
pub mod sub_test3;