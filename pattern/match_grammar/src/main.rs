fn main() {
    // 直接匹配字面量值， 将字面量 1 赋值给 x   
    // let x = 1;
    let x = 5;

    match x {
        // 使用 1 | 2 表达式 表明 该分支可以匹配 1 或者 2
        1 | 2=> println!("one or two"),
        // 使用 3 ..= 5 表示该分支匹配 3至5范围内的值
        3 ..= 5 => println!("three to five"),
        _ => println!("anything"),
    }

    let x = Some(5);

    match x {
        Some(50) => println!("Got 50"),
        // x 的 模式与 下列表达式匹配，应该根据命名变量匹配原则，x 中的参数 5 被赋值给 变量 y
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    struct_parse();

    enum_parse();

    test_point();

    foo(1,2);
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}


fn struct_parse() {
    let p = Point { x: 0, y: 7 };

    // 通过结构体解析语法，将 p 对象 字段 x 值赋值给 a 变量，字段 y 值 赋值给 b 变量
    let Point { x: a, y: b } = p.clone();
    println!("a = {}", a);
    println!("b = {}", b);

    // 使用简化结构体解析语法，获取结构体中对应数据
    let Point { x, y } = p.clone();
    println!("x = {}", x);
    println!("y = {}", y);

    match p {
        // 匹配 任何 字段 y 值为 0 的分支
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // 匹配 任何 字段 x 值为 0 的分支
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // 匹配 所有类型为 Point 的对象
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // 解析 元组与 结构体嵌套
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("(({}, {}), Point  {}, {} )", feet, inches, x, y);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_parse() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // 通过匹配 Message 中 的 Move 元素，来将对应数据赋值 给 x 与 y 变量
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        // 通过匹配 Message 中 的 Write 元素，来将对应数据赋值给 text 变量
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // 通过匹配 Message 中 的 ChangeColor 元素，来将对应数据赋值给 r, g, b 变量
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

// 表明 忽略 函数传参的第一个值，在本函数中不会使用
fn foo(_: i32, y: i32) {
    // _ 不能使用
    // println!("This code only uses the y parameter: {}, {}", y, _);
    println!("This code only uses the y parameter: {}", y);
}

fn test_point() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 使用 @ 运算符 表示 第一个值 是否属于 1到5的范围，属于的话 将值赋予 first
        (first @ 1..=5, ..) => {
            println!("first @ 1..=5: {first}");
        },
        // 匹配第一个和最后一个元素 同时满足 first == 2 情况下 执行
        (first, .., last) if first == 2 => {
            println!("Some numbers and first == 2: {first}, {last}");
        },
        // 匹配第一个和最后一个元素
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    match numbers {
        // 匹配第一个元素
        (first, ..) => {
            println!("Some numbers: {first}");
        }
    }

    match numbers {
        // 匹配最后一个元素
        (.., last) => {
            println!("Some numbers: {last}");
        }
    }

    // match numbers {
        // 由于无法确定 second 具体位置， 因此会报错
    //     (.., second, ..) => {
    //         println!("Some numbers: {last}");
    //     }
    // }
}