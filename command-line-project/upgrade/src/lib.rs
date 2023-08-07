use std::error::Error;
use std::fs;

// 使用config中的file_path字段值处理逻辑
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

// 使用 config 结构接收命令行参数
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 使用 Config 的构造函数对接收参数进行处理
    // 将对应参数命名，同时决定那个参数应该
    fn new (args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config{
            query,
            file_path
        }
    }

    // 使用 build 函数调用验证 并返回对应错误
    pub fn build(args: &[String]) -> Result<Config, String> {
        // 验证 
        if args.len() < 3 {
            return Err(format!("not enough arguments"));
        }
        Ok(Config::new(args))
    }
}