// 使用 use 将 sub_test3 引入本作用域
// use test1::sub_test3;

// 使用 as 关键字可以对 sub_test3 指定别名
// use test1::sub_test3 as other_test;
// 使用 use 将 sub_test3 中的 Color枚举 引入本作用域
// use test1::sub_test3::Color;

// 由于上述两条use语句有通用前缀 test1 因此可以使用嵌套路径进行合并
// use test1::{sub_test3 as other_test, sub_test3::Color};

// 使用 use test1::sub_test3::* 可以将 test1::sub_test3 中所有公有项引入
use test1::sub_test3::*;


// 使用内联的方式声明模块
mod test{
    // 为了使 hosting 子模块能被外部代码访问，需要添加 pub 关键字
    pub mod hosting {
        // 为了使 add_to_waitlist 函数能被外部代码访问，需要添加 pub 关键字
        pub fn add_to_waitlist() {
            // 由于 sub_test3 模块只是引入了 main.rs 所在作用域，未引入 hosting 作用域 因此下列代码会报错
            // let car = sub_test3::Car::new(String::from("73j64"), sub_test3::Color::White, String::from("斯柯达"));
            println!("add_to_waitlist 方法被调用了");
        }
    }
}

// 该声明模块指向src/test1.rs 文件
mod test1;

// 该声明模块指向src/test2/mod.rs 文件
mod test2;

mod test3;

fn main() {
    // 使用绝对路径调用 test 模块中的 hosting 子模块中的 add_to_waitlist 函数
    crate::test::hosting::add_to_waitlist();
    // 使用相对路径调用 test1 模块中的 sub_test3 子模块中的 print_type 函数
    test3::test_print("1111");

    let mut user = test3::User::new(&String::from("张三"), &String::from("成都"), true);

    // 由于 User 结构体中 username 使用 pub 修饰 因此下述 语句成功
    user.username = String::from("李四");
    // 由于 User 结构体中 address 没有使用 pub 修饰 对本模块不可见 因此下述 语句报错
    // user.adress = String::from("上海");

    // 由于 sub_test3 模块已经引入，因此可以直接使用 模块名::调用代码 的方式使用 sub_test3 中的代码
    // 由于 sub_test3 中的 Color 枚举 已被引入， 因此可以直接使用 Color 而不需加上 sub_test3 前缀 对 Color 枚举进行调用
    // let car = sub_test3::Car::new(String::from("73j64"), Color::White, String::from("斯柯达"));

    // 在 sub_test3 模块 已经使用别名 other_test 的情况下 可以使用 other_test 替代之前的 sub_test3
    // let car = other_test::Car::new(String::from("73j64"), Color::White, String::from("斯柯达"));

    // 由于已经使用 use test1::sub_test3::*; 将 test1::sub_test3 路径中的所有共有项引入，因此可以直接使用 sub_test3 中的 Car 以及 Color
    let car = Car::new(String::from("73j64"), Color::White, String::from("斯柯达"));
    
    println!("{}", car.get_brand())
}
