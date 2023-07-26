fn main() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        // result 获取 string1 与 string2 的生命周期交集，及string2的生命周期
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
        // 离开 作用域时 string2 生命周期结束，同样result的生命周期同时结束
    }
    // 由于 result 生命周期已结束，因此 result 在此处已失效，下列语句编译时会报错
    // println!("The longest string is {}", result);

    let i;

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        // 由于ImportantExcerpt 的生命周期与 novel 一致
        i = ImportantExcerpt {
            part: first_sentence,
        };
        // novel 依然有效 因此 i 有效
        println!("{:?}", i);
    }

    // 离开 novel 作用域后 i 也一起失效了，因此下条语句会报错
    // println!("{:?}", i);
}

// 由于 参数 x 与 y 使用生命周期注解 'a 修饰，因此 'a 的生命周期为传入 x 与 y参数生命周期交集 
// 返回值带有 'a 注解则表示 返回值具有和 'a 一致的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 使用 'a 修饰 part 字段 表示 'a 与 part 具有一样的生命周期
// 本结构体的实例化对象与 'a 的生命周期一致
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
