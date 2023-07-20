use std::{fs::File, io::{ErrorKind,Error,Read}};

fn main() {
    // File Open 函数 返回值为Result 枚举
    // let greeting_file_result = File::open("hello.txt");

    // 使用 match 处理执行结果
    // match greeting_file_result {
        // Result::Ok(t) => println!("打开成功"),
        // 使用 e.kind() 获取 错误类型枚举
        // Result::Err(e) => match e.kind() {
            // 对 ErrorKind::NotFound 采取对应逻辑
            // ErrorKind::NotFound => panic!("未找到文件"),
            // 其他情况下使用其他逻辑
    //         _ => panic!("由于其他错误，未打开文件")
    //     }
    // }

    // unwrap方法 执行成功返回对象，失败时则是执行对应的panic；
    // let greeting_file_result = File::open("hello.txt").unwrap();

    // expect方法 执行成功返回对象，失败时则是执行对应的panic 替换panic message；
    // let greeting_file_result = File::open("hello.txt").expect("自定义错误");
    // 处理 open_file 传递的错误
    let greeting_file_result = open_file(&String::from("hello.txt")).expect("传递的自定义错误");
}

fn open_file(file_name: &str) -> Result<File, Error> {
    // 在本函数中不想处理该错误时，直接将Result 作为返回值传递给调用者处理
    File::open(file_name)
}

fn read_username_from_file() -> Result<String, Error> {
    // 使用 ？运算符结尾时表示如果 File::open("hello.txt") 出现错误直接返回该错误
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // 使用 ？运算符结尾时表示如果 read_to_string(&mut username) 出现错误直接返回该错误
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
