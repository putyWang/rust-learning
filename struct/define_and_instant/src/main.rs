fn main() {
    println!("Hello, world!");

    // 使用 结构名 {key:value,key:value,...} 对 UserStruct 进行实例化 并且赋值给 user
    let mut user = UserStruct{
        // 表示 将 String::from("张三") 赋值给 user 的 username 字段
        username: String::from("张三"),
        adress: String::from("成都"),
        sex: 1,
        active:true
    };

    // 使用 结构对象名.字段名的方式访问对应字段值
    println!("{}",user.username);
    // 实例化对象可变的情况下，能够使用 实例化对象名.字段名 的方式给对应字段赋值
    user.sex = 2;
    println!("{}",user.sex);

    let user2 = build_user(String::from("李四"), String::from("北京"), 2);
    println!("{}",user2.username);

    // 当 user3 与 user2 之间只有 username 不一致时，使用 结构名{差异字段赋值, ..旧实例名} 进行赋值
    let user3 = UserStruct{
        // 对 用户名进行赋值
        username:String::from("王五"),
        // 剩余字段值与user2一致
        ..user2
    };

    println!("{}",user3.username);
    // 由于在 user3 示例化时 使用了 ..user2 导致 user2 对象中的String类型的值产生了移动，从而导致user2中的String失效，下列语句会报错
    // println!("user2 adress 字段值为{}",user2.adress);
    // 基本数据类型的字段 sex 则不会产生移动，因此依然有效
    println!("user2 adress 字段值为{}",user2.sex);
    println!("user3 adress 字段值为{}",user3.adress);
    // 下一处对已移动的对象值重新赋值，使得再次访问该值不会出错
    user2.adress = String::from("上海");
    println!("user2 adress 字段值为{}",user2.adress);
}

// 使用 struct 定义 user 结构
// 结构体命名采取首字母大写的驼峰命名方式
struct UserStruct{
    // 定义数据类型为 String 的 username 字段
    username: String,
    adress: String,
    sex: i16,
    active: bool
}

fn build_user (username: String, adress: String, sex: i16) -> UserStruct {
    UserStruct{
        // 若参数名与字段名一致，则可以省略 :参数名 直接使用字段名对指定字段赋值
        // 由于 usename adress sex 三个字段与参数名一致，因此都采取简写的方式
        username,
        adress,
        sex,
        active: true
    }
}
