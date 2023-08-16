// 声明 类型为 &str 的全局变量
static HELLO_WORLD: &str = "Hello, world!";
// 声明 类型为 u32 的可变静态变量
static mut COUNTER: u32 = 0;

fn main() {
    unsafe_rust();
    // 由于 dangerous() 是不安全函数，因此在安全区域内调用无法通过编译
    // dangerous();

    println!("r1 is: {}", HELLO_WORLD);
    
    unsafe {
        // 在不安全块调用 不安全函数 可以通过编译
        dangerous();
        // 调用 extern 块中函数
        // my_c_function(1);
    }

    test_mut_static_value();
    
}


fn unsafe_rust(){
    let mut num = 5;

    // 创建指向 num 的不可变裸指针
    let r1 = &num as *const i32;
    // 创建指向 num 的可变裸指针
    let r2 = &mut num as *mut i32;

    // 由于只能在不安全块中解引用裸指针，因此下行会编译失败
    // println!("r1 is: {}", *r1);

    // unsafe 关键字 表明 修饰的块为不安全块
    unsafe {
        // 由于在不安全块中解引用裸指针，因此下行没问题
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {}

// 使用 link 参数 与 extern 关键字表明将在运行时尝试与类 Unix 系统上的 libmy_c_library.so 和 Windows 上的 my_c_library.dll 链接
// 链接失败时 会链接到 panic 报错
// #[link(name = "my_c_library")]
// extern "C" {
//     // 声明需使用的函数
//     fn my_c_function(x: i32) -> bool;
// }

// 使用 extern 关键字创建一个其他函数调用的 call_from_c() 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn test_mut_static_value(){
    // 在安全区域对可变静态变量进行修改与访问是不允许的
    // COUNTER += inc;
    // println!("COUNTER: {}", COUNTER);

    unsafe {
        // 修改可变静态变量
        COUNTER += 3;
        // 访问可变静态变量
        println!("COUNTER: {}", COUNTER);
    }
}