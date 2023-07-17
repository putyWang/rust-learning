fn main() {
    // 使用 new() 关联函数创建 String 对象
    let mut s1:String = String::new();
    // 使用 from(字符串字面量) 关联函数创建 String 对象
    let mut s2:String = String::from("s2");
    // 使用 字符串字面量.to_string() 关联函数创建 String 对象
    let s3:String = "s3".to_string();

    println!("s1的值为{}", s1);
    println!("s2的值为{}", s2);
    println!("s3的值为{}", s3);
    // 使用 push_str(字面量) 为s1添加后缀
    s1.push_str("s1新后缀");
    println!("s1的值为{}", s1);
    // 使用 push(字符) 为s2添加后缀
    s2.push('2');
    println!("s2的值为{}", s2);
    // 使用运算符拼接字符串
    let s4 = s2 + &s3;
    // s2 已经移动了 导致 s2 失效
    // 使用 format!宏 定义s5
    let s5 = format!("s1-s3-s4的值为{}-{}-{}", s1, s3, s4);
    // 宏的格式与 println!宏 一致，因此下列两行输出一致
    println!("s1-s3-s4的值为{}-{}-{}", s1, s3, s4);
    println!("{}", s5);
    // 使用 chars() 方法将s5转化为 字符数组，然后进行遍历
    println!("以char对s5进行遍历");
    for i in s5.chars(){
        println!("{i}");
    }
    // 使用 bytes() 方法将s5转化为 字节数组，然后进行遍历
    println!("以byte对s5进行遍历");
    for i in s5.bytes(){
        println!("{i}");
    }
}
