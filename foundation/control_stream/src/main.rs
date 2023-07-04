fn main() {
    println!("Hello, world!");

    println!("{}", compare_to(1, 2));

    println!("1 是 {}, 2 是 {}", is_odd_str(1), is_odd_str(2));

    temp();

    iterate_through_array([1, 2, 3, 3]);
}

fn compare_to (x: i32, y: i32) ->  &'static str { 
    let dvi = x - y;

    // 直接在let 语句中使用if语句声明变量result值
    // if 表达式 以 if开头
    let result = if dvi < 0 {
        "x < y"
    // 存在多个分支条件时 使用else if 做余量判断
    }else if dvi == 0 {
        "x = y"
    // 使用else 处理余量
    }else {
        "x > y"
    };

    result
}

fn is_odd(x: i32) -> bool { 
    let mut y = x;
    // loop 循环直到 >10 退出
    // loop {
    //     y += 2;
    //     // 当 y < 10 时，使用continue 跳过之后的语句 直接进入下一次循环
    //     if y < 10 {
    //         continue;
    //     }

    //     // 一旦 y >= 10 直接跳出整个循环
    //     break;
        
    // }

    // 也可使用 while 循环
    // 在 y < 10 的条件下一直执行 y += 2 直到 y >= 10
    while y < 10 {
        y += 2; 
    }

    println!("结束y<10的循环 当前 y 为{y}");


    loop {
        y += 2;
        // 当 y > 20 时，使用break 表达式获取loop返回值并退出循环
        if y > 20 {
            println!("结束y<20的循环 当前 y 为{y}");
            break y % 2 == 1;
        }
    }
}

fn is_odd_str(x: i32) -> &'static str { 
    if is_odd(x) {
        "奇数"
    } else {
        "偶数"
    }
}

fn temp () {
    let mut count = 0;
    // 为外层循环打上 counting_up 标签
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // 不指定标签跳出未打标签循环
                break; 
            }
            if count == 2 {
                // 使用 break 标签名的方式 可以指定跳出的具体循环
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn iterate_through_array(array: [i32; 4]) {
    // 使用for循环遍历array
    for i in array {
        println!("array 个元素为{i}");
    }
}
