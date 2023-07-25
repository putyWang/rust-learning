use std::cmp::PartialOrd;
use std::marker::Copy;

fn main() {
    let mut num_vec = vec![1,2,3,4];
    let mut char_vec = vec!["a", "b", "c", "d", "e"];

    // 由于使用泛型替代了具体类型，因此可以传递实现了 PartialOrd 的类型参数
    println!("{}",largest(&mut char_vec));
    println!("{}",largest(&mut num_vec));

    // 显示声明 Point 结构 T 具体类型为 i32， R 具体类型为 f32
    let point = Point::<i32, f32>{
        x: 1,
        y: 2,
        ratio: 1.0
    };
    println!("{:?}", point);
    // 程序通过 Point 中字段具体值推测 T 实际类型为 f32 与 R 实际类型为 &str
    let point2 = Point{
        x: 12.0,
        y: 12.0,
        ratio: "12%"
    };
    println!("{:?}", point2);
    println!("{}", point2.get_x());

    // 声明指定泛型的枚举对象
    let result = Result::ok("成功");
    let result2 = Result::ok(200);
}


// 使用 <T> 定义泛型名 为 T
// 在参数列表中声明传参类型为 &[T] 
// 返回值类型也声明为泛型T
fn largest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 定义泛型 x y 字段类型全为T的泛型
#[derive(Debug)]
struct Point<T:Copy, R> {
    x:T,
    y:T,
    ratio:R
}

// 在 impl 块上定义泛型
impl<T:Copy, R> Point<T,R>{
    // 方法中直接使用
    fn get_x(&self) -> T {
        self.x
    }
}

// 定义Result枚举的泛型
enum Result<T> {
    ok(T),
    fail
}


