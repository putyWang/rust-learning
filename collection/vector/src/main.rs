fn main() {
    // 通过 Vec 的 new 关联函数声明一个 vec 对象；
    // Vec 中的元素类型为 i32
    let v:Vec<i32> = Vec::new();

    // 使用 vec!宏 的方式声明 v 为 [1, 2, 3] 的Vector 对象
    // 不用声明变量 v 数据类型，直接由程序推断为 Vec<i32>;
    let mut v = vec![1, 2, 3];
    // 删除index 为0数据
    v.remove(0);
    // 向v 尾部添加数据 4
    v.push(4);

    // 使用 pop() 获取 v 的尾部数据，同时删除该数据
    // 由于 vector 中元素使用 Option进行包装
    if let Option::Some(last) = v.pop() {
        println!("{}", last);
    }

    if let Option::Some(last) = v.get(0) {
        println!("{}", last);
    }

    // 由于 vector 现在大小为 2 因此 index 只有在 0 和 1 有数据，所以 get(4) 结果为 Option::None
    if let Option::None = v.get(4) {
        println!("为空");
    }

    // 使用 &v[1] 直接获取 index 为 1 的 i32 类型数据
    let second = &mut v[1];
    *second = 12;
    println!("{}", second);
    // 当使用 pop 方法向对象中添加数据后， second 引用直接失效
    // if let Option::Some(last) = v.pop() {
    //     println!("{}", last);
    // }

    // println!("{}", second);
    // 当使用 push 方法向对象中添加数据后， second 引用也是直接失效
    // v.push(2);
    // println!("{}", second);
    // 当index为 4 时 直接报错；
    // &v[4];
    // 使用 for 元素名 in &对象名 遍历整个对象元素
    for i in &v {
        println!("{}", i);
    }
}
