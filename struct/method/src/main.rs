fn main() {
    // 使用 结构名 {key:value,key:value,...} 对 UserStruct 进行实例化 并且赋值给 user
    let mut user = UserStruct{
        // 表示 将 String::from("张三") 赋值给 user 的 username 字段
        username: String::from("张三"),
        adress: String::from("成都"),
        sex: 1,
        active:true
    };

    // 使用结构体实例名.方法名的方式对方法进行调用
    // 调用方法时，需传除 &self 以外的所有参数
    user.println();
    let change = String::from("李四");
    user.chang_name(change);
    user.println();
    // 使用 :: 双冒号调用 UserStruct 关联 new 构造函数方法，创建对应构造体实例对象
    let user2 = UserStruct::new(String::from("李四"), String::from("北京"), 2);
    user2.println();
}

struct UserStruct{
    username: String,
    adress: String,
    sex: i16,
    active: bool
}

// 定义方法上下文为 UserStruct
impl UserStruct {
    // 定义不带其他参数的方法
    fn println(&self) {
        println!("当前用户名为{}，籍贯{}，性别{}，是否启用：{}", self.username, self.adress, self.sex, self.active);
    }

    // 定义带其他参数的方法
    // 由于该方法中会改变 self 字段参数， 因此需要加上 mut 前缀
    fn chang_name(&mut self, username:String) {
        self.username = username;
    }
}

// 第二个impl块
impl UserStruct {
    // 关联构造方法，Self 表示返回值为关联UserStruct
    fn new(username: String, adress: String, sex: i16) -> Self {
        Self{
            username,
            adress,
            sex,
            active:false
        }
    }
}