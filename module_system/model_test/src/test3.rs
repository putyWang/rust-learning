pub fn test_print(s: &str) {
    // 使用绝对路径调用 test1 模块中的 sub_test3 子模块中的 print_type 函数
    crate::test1::sub_test3::print_type(s);
    // 使用 super 替换 根节点模块
    super::test1::sub_test3::print_type(s);
}

// 声明 User 结构体为公用的
pub struct User {
    // username 声明为公用
    pub username: String,
    adress: String,
    active: bool
}

impl User {
    pub fn change_address(&mut self, adress:&String) {
        self.adress = adress.to_string();
    }
}

impl User {
    pub fn new (username:&String, adress:&String, active:bool) -> Self {
        Self{
            username: username.to_string(),
            adress: adress.to_string(),
            active: active
        }
    }
}

// 使用pub修饰的枚举可以访问
pub enum IsActive (
    Enable,
    NotEnable
)