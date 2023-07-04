fn main() {
    //使用 函数名(参数列表) 调用函数
    server_for_main(server_for_main(server_for_main(1)));
}

/**
 * fn 关键字：用于定义函数
 * server_for_main：snake case 方式编写的函数名；
 * service_type：类型为i32的参数；
 * -> i32 表示返回值类型为 i32
 */
fn server_for_main (service_type:i32 ) -> i32 {
    // 多行没有返回值的语句
    if service_type == 1 {
        println!("正在调用服务1");
    }else if service_type == 2 {
        println!("正在调用服务2");
    }else {
        println!("该服务不存在");
    }

    // 表达式表达返回值为 service_type + 1
    // 表达式结尾不以 ; 结尾，否则将不是返回值表达式，仅仅时一行普通语句
        service_type + 1
}
