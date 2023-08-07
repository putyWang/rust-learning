fn main() {
    // 闭包的定义
    define_closure();
    // 捕获不可变变量
    un_mut_closures();
    // 捕获可变变量
    mut_closures();
    // move 关键字的使用
    use_move();
}

fn define_closure(){
    // 以下三个变量指定的右边代码都是等价的 都是指定传递给闭包一个 u32 类型的变量 同时返回一个 u32 类型的值
    // add_one_v2 为完整写法
    let add_one_v2 = |x: u32| -> u32 { x };
    // add_one_v3 省略了 传参类型与 返回值类型
    let add_one_v3 = |x| { x };
    // add_one_v4 省略了 花括号
    // let add_one_v4 = |x| x ;
    // 对 add_one_v2 进行调用
    println!("{}", add_one_v2(1));
    // 第一次对 add_one_v3 调用时 对 使用确定了 x 的默认类型为 i32
    println!("{}", add_one_v3(2));
    // 之后对 add_one_v3 也必须传递 i32 类型的数据
    // 下一行中向 add_one_v3 中传递 String 类型的参数就会报错
    // println!("{}", add_one_v3(String::from("1111111111111")));
}

fn un_mut_closures(){
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {:?}", list);
    // 由于list 为 不可变对象，因此在 声明闭包与 使用闭包之间可以对list 进行操作
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mut_closures(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // 在定义闭包与最后一次使用闭包之间不允许对可变引用对象进行操作
    // println!("Before calling closure: {:?}", list);
    // 最后一次使用 闭包
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn use_move(){
    let list = vec![1, 2, 3];
    // 使用 move 关键字后强行借用对象所有权 直到最后一次使用
    let only_borrows = move || println!("From closure: {:?}", list);
    // 由于使用 move 关键字 强行获取 list 的所有权，因此 list 变量失效
    // println!("Before calling closure: {:?}", list);
    only_borrows();
    // 由于使用 move 关键字 强行获取 list 的所有权，因此 list 变量失效
    // println!("After calling closure: {:?}", list);
}