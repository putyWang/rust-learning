fn main() {
    // 下列行使用 panic! 宏 手动调用panic 进行 panic 相关操作；
    // panic!("111111111111");

    let v = vec![1, 2, 3];
    // 由于对象 v 长度只有3，因此index 为 2 时能访问到
    println!("{}", &v[2]);
    // 由于对象 v 长度只有3，因此index 为 100 时超过 2 所以系统自动执行 panic! 宏 抛出 index out of bounds 异常
    println!("{}", &v[99]);
}
