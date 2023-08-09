use crate::ConsList::Nil;
use std::ops::Deref;

fn main() {
    // 创建指向堆上 123 数据的 box 指针
    let b = Box::new(123);
    // 直接访问 b 指针数据
    println!("{}", b);
}

// cons list 表示 ConsList 嵌套 ConsList 的递归类型
enum ConsList {
    // 由于ConsList时递归类型，不确定大小，因此不能直接创建
    // Cons(i32, ConsList),
    // 由于ConsList时递归类型，使用Box指针创建指向ConList对象的指针
    cons(i32, Box<ConsList>),
    Nil,
}

struct MyBox<T>(T);

// 自定义指针必须 实现 Deref trait 才能被程序解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

// CustomSmartPointer 实现了 Drop trait
impl Drop for CustomSmartPointer {
    // 实现了 drop 方法
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::drop;

    #[test]
    fn test_dereference_operator(){
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        // 由于 y 是 5 的引用，y与5数据类型不一致，因此直接对比出错
        // assert_eq!(5, y);
        // 使用解引用运算符后能获取到y引用的值，因此可以直接与 原值5 进行对比
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box() {
        let b = Box::new(5);
        // box 指针也能够使用解引用运算符获取到 封装的原值
        assert_eq!(5, *b);
        // 直接对比也 box 指针与原值 与 引用一致 会出现相同的问题
        // assert_eq!(5, b);
    }

    #[test]
    fn test_my_box() {
        let b = MyBox::new(5);
        // box 指针也能够使用解引用运算符获取到 封装的原值
        assert_eq!(5, *b);
        // 直接对比也 box 指针与原值 与 引用一致 会出现相同的问题
        // assert_eq!(5, b);
    }

    #[test]
    fn test_cons_list (){
        // 由于ConsList时递归类型，不确定大小，因此不能直接创建
        let list_box = ConsList::cons(1, Box::new(ConsList::cons(2, Box::new(ConsList::cons(3, Box::new(Nil))))));
    }

    #[test]
    fn test_deref_coercions(){
        fn main() {
            let m = MyBox::new(String::from("Rust"));
            // hello 函数形参类型为 &str 实现了 Deref trait，因此可以传递 同样实现了 Deref trait 的 Box ，因此在传参过程中使用了强制类型转换
            hello(&m);
        }
    }

    #[test]
    fn test_drop(){
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
        // 无法手动调用 drop 方法提前清理变量，因此 下列通过 c.drop() 的方式手动调用出错
        // c.drop();
        // 使用 drop 函数手动回收对象 c 的内存
        drop(c);
        println!("CustomSmartPointers will be distoried.");
        // 在离开作用域时，c与d被回收
        // 由于 CustomSmartPointer 实现了 drop 方法 因此离开作用域时自动调用对应drop方法
    }
}
