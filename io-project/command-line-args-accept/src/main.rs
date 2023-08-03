use std::env;
use std::fs;

fn main() {
    // 通过std::env::args().collect() 获取命令行传参集合
    let args: Vec<String> = env::args().collect();

    let file_path = &args[2];

    println!("In file {}", file_path);
    // 读取指定传入路径的文件
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
