fn main() {
    /*
     * 声明长度为 16 的无符号整型变量 _x 
     * 数长度过长时允许使用下划线"_"进行分隔
     */
    let mut _x:u16 = 11_111;
    // 可使用不同表示的字面量对数进行标识（10进制， 16 进制，8 进制）
    _x = 0xffff;
    println!("x 值为 {_x}");
    // 默认声明小数为f64
    let f = 0.01;
    print_type_of("char default", &f);
    // 声明f32类型的小数需指出类型
    let f:f32 = 0.1;
    // 声明boolean类型数据
    let yes = true;
    // 声明char类型数据（用单引号包裹的字面量默认声明为char）
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
    // 显示声明元组类型
    let tup: (i16, f32, char) = (21, 0.22, 't');
    // 解构元组
    let (copy1, copy2, copy3) = tup;
    println!("元组tup属性为({copy1},{copy2},{copy3})");
    // 也可以不指定元组数据类型，由系统根据字面类型进行赋值
    let tup = (0.33, 1245, 'u', "123135");
    // 直接访问元组中的数据
    println!("元组tup属性为({},{},{},{})", tup.0, tup.1, tup.2, tup.3);
    // 声明单元元组
    let _null = ();
    // 使用[i16;4]声明长度为4的i16类型的数组
    let array:[i16;4] = [1,2,3,4];
    // 声明长度为3的i32类型的数组
    let array = [1,2,3];
    print_type_of("array[1] defalut", &array[1]);
    // 声明长度为4元素值全为2的数组
    let array = [2;4];
    // 使用 数组名[index] 对访问指定数组元素
    println!("array is [{},{},{},{}]", array[0], array[1], array[2], array[3]);
}

/**
 * 打印变量数据类型函数
 */
fn print_type_of<T>(name:&str, _: &T) {
    println!("type of {} is {}", name, std::any::type_name::<T>())

}
