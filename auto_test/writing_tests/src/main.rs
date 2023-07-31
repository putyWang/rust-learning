fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test] 注解表明 it_works 函数为一个测试函数
    // 在对项目使用 cargo test 命令时，会运行该函数
    #[test]
    fn it_works() {
        let result = 2 + 2;
        // 使用 assert_eq!宏 对比结果是否相同
        assert_eq!(result, 4);
    }

    #[test]
    // #[ignore] 表示运行时忽略该测试
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        println!("测试larger: {:?}，测试smaller: {:?}", larger, smaller);

        // 使用 assert! 宏 检查 larger can_hold(&smaller) 方法的运行结果
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        // 使用 assert_eq! 宏 对比 4 以及 add_two(2) 计算结果 是否相等
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // 该处使用自定义异常信息的方式打印测试失败时字符串（采用 format!宏 的方式拼接第二个与第三个参数）
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // 使用 #[should_panic] 注解测试 greater_than_100() 函数是否按照要求 进行 panic
    // 在需要进一步规范 panic 具体内容时，可以通过设置 expected 参数来检查 panic 出的 message 是否包含指定文本；
    #[should_panic(expected = "Guess value must be between 1 and 100, got")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            println!("打印：Guess value must be between 1 and 100, got {}.", value);
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}