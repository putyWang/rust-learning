fn main() {
    // 若需要对 x 进行二次赋值 需要使用 mut 进行修饰 
    // let mut x = 5;
    // const 关键字与申明常量 也是不可变的 const x = 5;
    let x = 5;
    println!("The value of x is: {x}");
    // 本次声明x新值对之前申明值进行了隐藏
    let x = 6;
    {
        // 本次声明x新值对之前申明值进行了隐藏
        let x = x * 2;
        println!("The value of x is: {x}");
        // 相当于重新声明了 x 因此可以与之前声明的整型不一致使用字符串类型
        let x = "1321321";
        println!("The value of x is: {x}");
        //作用域结束后 结束对 x = "1321321" 值的隐藏
    }
    // x 未用 mut修饰时，x = 6 重新赋值编译时报错 cannot assign twice to immutable variable `x`
    // x = 6;
    println!("The value of x is: {x}");
}
