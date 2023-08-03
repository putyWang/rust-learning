use std::env;
use std::process;
use upgrade::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 转换过程中 错误在 主函数中进行处理抛出
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 调用 lib.rs 中的 run 函数
    if let Err(e) = upgrade::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
