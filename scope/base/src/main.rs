fn main() {
    println!("Hello, world!");
    {
        let s = String::from("hello"); // 从这个地方开始 变量s是有效的，并且申请变量所需内存；
        // s 指向的指针被移动到 new 对象中
        // 变量s 值移动到new 导致变量 s 指针失效
        let new = s; 
        // 在变量移动之后，s 无效 因此在使用 s 时会报错；
        // println!("{}", s); 
        println!("{}", new); 
        // 使用clone时，copy存储着新克隆堆中数据
        let copy = new.clone();
    } // 在这个地方 作用域已经结束，该作用域中的所有变量 全部失效，同时释放对应内存；
    
    let s = String::from("hello");
    // 此处 s 移动到了print函数的作用域中，导致失效；
    // 由于出现了参数的移动，因此程序编译出错（borrow of moved value: `s`）提示你这个地方产生了移动，
    // print(s); 
    // 需要使用clone的方式进行传参
    print(s.clone()); 
    // 对于实现了copy的数据，则可直接赋值
    let not_copy = 1;
    print_int(not_copy);

    println!("{}", gives_ownership());
}


fn print(s: String) {
    println!("{}", s);
}

fn print_int(not_copy: i32) {
    println!("{}", not_copy);
}

fn gives_ownership() -> String {
// some_string 进入作用域
let some_string = String::from("yours"); 
// 返回值移动给调用它的函数
some_string 
}