use std::collections::HashMap;

fn main() {
    // 创建HashMap时显示声明 key 类型为 String, value 类型为 i32
    let mut map:HashMap<String,i32> = HashMap::new();
    map.insert(String::from("frist"), 1);
    // 创建HashMap时未声明 key 与 value 类型
    let mut map_new = HashMap::new();
    // 在第一次使用 insert 方法向 HashMap对象中插入数据时，程序推测出 key的类型为 i32， value 类型为 i32
    map_new.insert(1,1);
    map_new.insert(2,2);
    map_new.insert(3,3);
    map_new.insert(4,4);
    // 由于在第一次插入时已规定了 key 与 value 数据类型，因此下面 map_new.insert("2",1) 语句无法编译；
    // map_new.insert("2",1);
    // 使用 get(&String::from("frist")) 函数，获取map 中 get(&String::from("frist")) key 对应的 value（使用 Option 枚举包装之后的）
    let frist = map.get(&String::from("frist"));
    print(frist);
    // 由于 map 对象中不存在 get(&String::from("second")) 这个key 因此 second 值应该为 Option::None
    let second = map.get(&String::from("second"));
    print(second);
    for_print(&map);
    // 此处由于之前不存在 String::from("second") 这个值, 因此 insert 方法添加了一对新参数
    map.insert(String::from("second"), 20);
    for_print(&map);
    // 此处由于之前已存在 String::from("second") 这个值, 因此 insert 方法只是更新了对应的value值
    map.insert(String::from("second"), 30);
    for_print(&map);
    // 此处由于之前已存在 String::from("second") 这个值, 因此 or_insert(40) 方法未更新了对应的value值
    map.entry(String::from("second")).or_insert(40);
    for_print(&map);
    // 此处由于之前不存在 String::from("thred") 这个值, 因此 or_insert(40) 方法插入了 (String::from("three"), 40) 这个键值对
    map.entry(String::from("thred")).or_insert(40);
    for_print(&map);
    // 此处由于之前已存在 String::from("second") 这个值的可变引用
    let second = map.entry(String::from("second")).or_insert(40);
    // 因此可以直接改变对应值
    *second = 50;
    for_print(&map);
}

fn print<T: std::fmt::Display> (value:Option<T>) {
    match value{
        Option::Some(i) =>  {
            println!("该值为{}", i);
        },
        Option::None => {
            println!("该值不存在");
        }
    } 
}

fn for_print<K: std::fmt::Display, V: std::fmt::Display>(map:&HashMap<K, V>) {
    println!("-------------------------------");
    for (key, value) in map {
        println!("{key}-{value}");
    }
}
