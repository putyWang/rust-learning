pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    a + a
}

// 使用 #[cfg(test)] 表明 tests 模块 为本单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 测试模块中运行直接访问私有函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
