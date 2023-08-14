fn main() {
    // let 语法中的模式匹配是不可反驳的
    // 因此 = 号 左右的参数模式必须匹配，否则会报错
    // 由于 = 左边模式为 some 枚举， 右边为 None，不匹配因此在不可反驳模式下会报错
    let x:Option<i32> = None;
    // let Some(x) = x;

    // 使用 可反驳模式语句 if let 两边模式匹配不上则不会报错
    if let Some(x) = x {
        println!("{}", x);
    }else {
        println!("模式匹配失败");
    }
}
