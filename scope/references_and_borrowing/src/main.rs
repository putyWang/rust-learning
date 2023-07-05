fn main() {
    // 声明 s 是可更改的
    let mut s = String::from("Hello");
    // &s传入一个s的引用
    print(&s);
    // 由于 s 的所有权没有变化，s 依然有效；因此之后可以再次使用s
    println!("{}", s);

    // &mut s 传入一个s对象的可变引用, 前提是 s 在声明时也为可变的
    change(&mut s);
    let mut1 = &mut s;
    let s1 = &s;
    let s2 = &s;
    // 不可变变量的声明 不会导致之前不可变变量失效
    // 但是可变变量会由于不可变变量的声明而失效
    print(s1);
    print(s2);
    // s 的可变引用 mut2 创建会导致之前所有创建引用失效，包括 mut1，s1, s2
    let mut2 = &mut s;
    // 由于 mut2 的创建会导致之前所有创建引用失效，包括 mut1，s1, s2;
    // 因此以下三条语句放开任意一条都会报错；
    // change(mut1); 
    // print(s1);
    // print(s2);
    change(mut2); 
    // 在可变引用使用过程中的修改会传导至s中
    println!("{}", s);
    dangle();
}

// &String表示需要传入一个 类型为String的数据的引用
fn print(s: &String) {
    // s.push_str(", world!");// 因为引用默认不能改变，因此该处对引用进行修改 放开会报错；
    println!("{}", s);
} // 作用域结束，由于s只是引用 没有对应数据的所有权，因此该数据不会被丢弃和清理

// &mut String 声明接收的参数为一个String类型的可变引用
fn change(s: &mut String) {
    s.push_str(", world!");// 由于s为可变引用，因此该处能够正常修改；
}

fn dangle() -> &String {
    let s = String::from("hello");
    // 由于 s 的数据是声明在函数中的，在离开本函数作用域时，s的数据将会被清理；
    // 因此返回 s 的引用将会报错
    &s
}