
// 使用内联的方式声明模块
mod test{
    // 为了使 hosting 子模块能被外部代码访问，需要添加 pub 关键字
    pub mod hosting {
        // 为了使 add_to_waitlist 函数能被外部代码访问，需要添加 pub 关键字
        pub fn add_to_waitlist() {
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
}
