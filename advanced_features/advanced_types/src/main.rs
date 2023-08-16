// 使用 type 关键字为 i32 类型 取 Kilometers 别名
type Kilometers = i32;
// 使用 Thunk 替代复杂格式
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    
    let y: Kilometers = 5;
    let x: i32 = 5;

    assert_eq!(x, y);

    let f: Thunk = Box::new(|| println!("hi"));
    let new_box = returns_long_type();
    new_box();
}


// 使用 new type 模式封装的 Vec<String>
struct Wrapper(Vec<String>);

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    Box::new(||{
        println!("调用");
    })
}