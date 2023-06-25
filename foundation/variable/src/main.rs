fn main() {
    // 若需要对 x 进行二次赋值 需要使用 mut 进行修饰 
    // let x = 5;
    // const 关键字与申明常量 也是不可变的 const x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    // x 未用 mut修饰时，编译时报错 cannot assign twice to immutable variable `x`
    x = 6;
    println!("The value of x is: {x}");
}
