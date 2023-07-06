fn main() {
    let mut s = String::from("hello world");

    // 使用 &s[0..5] 声明字符串 s 从 0 到 5 的一个字符串 slice
    let spilt1 = &s[0..5];
    println!("{spilt1}");
    // 使用 &s[6..11] 声明字符串 s 从 6 到 11 的一个字符串 slice
    let spilt2 = &s[6..11];
    println!("{spilt2}");
    // 由于spilt1 是从 0 开始的 因此可以省略 0 直接使用 &s[..5] 进行声明
    let spilt3 = &s[..5];
    println!("{spilt3}");
    // 由于 spilt2 中包含 s 的尾节点， 因此可以省略 字符串长度（11） 直接使用 &s[6..] 进行声明
    let spilt4 = &s[6..];
    println!("{spilt4}");
    // 对字符串的引用 相当于 对字符串的全量 slice 
    // 本例中 传参时 &s 与 &s[..] 其实是等价的
    let first_word_value = first_word(&s);
    println!("{}", first_word_value);
    // 源数据 s 出现任意改变时候会导致该数据绑定的 字符串slice 失效
    // s.clear(); 
    // s.push_str(", world");
    // 运行上述两行代码任意一行代码，s 将会出现更改，从而导致 first_word 等 字符串slice 失效，使得下述代码报错
    println!("the first word is: {}", first_word_value);

    // 字符串字面量 s2 的数据类型为 &str 即字符串 slice
    let s2 = "hello world";
    // 由于 s2 的数据类型为 &str 本身即是引用，因此可以直接作为 first_word 的传参
    println!("{}", first_word(s2));
}

// 字符串 slice 返回值 类型为 &str
// 由于对字符串的引用 相当于 对字符串的全量 slice ，因此该处参数使用 &str 可以有效兼容 &String 的传参
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}