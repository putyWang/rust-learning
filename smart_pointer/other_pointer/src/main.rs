use std::rc::Rc;
use crate::List::{Cons, Nil};
use std::cell::RefCell;

fn main() {
    test_rc();
    test_ref_cell();
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn test_rc(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 当前 a 对象，只有他本身一个引用指向他，因此当前引用计数为1
    println!("count after creating a = {}", Rc::strong_count(&a));
    // 使用 Rc::clone(&a) 新增了一个指向 a 的指针，并使得引用计数 + 1 变成2
    let b = Cons(3, Rc::clone(&a));
    // 当前 a 对象，他本身以及一个指针指向他，因此当前引用计数为2
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        // 使用 Rc::clone(&a) 又新增了一个指向 a 的指针，并使得引用计数再 + 1 变成3
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // 离开作用域时 c 被销毁 内存被回收，因此 c 中包含的 一个指向 a 的 Rc 指针也被销毁，因此当前 a 的引用计数 - 1 变成2
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}


fn test_ref_cell(){
    // a 声明为不可变变量
    // let a:Vec<i32> = vec![];
    // 由于 a 本身是不可变的，因此下列直接创建 a 的可变引用会报错
    // let b = &mut a;
    // 创建指向 a 的不可变 RefCell 指针
    let ref_cell = RefCell::new(vec![]);
    // 使用 ref 的 borrow_mut() 获取到封装值的可变引用
    let mut a = ref_cell.borrow_mut();
    // 可以对 对应的对象进行修改
    a.push(1);
    a.push(2);

    println!("{:?}", a);

}