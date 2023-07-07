fn main() {
    let mut user = UserStruct::new(String::from("张三"), Sex::man, String::from("成都"), true);
    user.print();
    user.set_sex(Sex::unkonw(String::from("LGBT")));
    user.print();

    let i = 3;

    // 配置 i32 数字
    let result = match i {
        // i == 1 时 返回 1
        1 => 1,
        // i == 2 时 返回 2
        2 => 2,
        // 其他情况下 返回 0
        // 也可使用 通配符"_" 代替 other
        other => 0
    };

    println!("{result}");

    // 当只需要一个分支时 可以使用 if let 的方式替代match
    let uokonw = Sex::unkonw(String::from("LGBT"));
    match uokonw {
        Sex::unkonw(ref s) => {
            println!("{} 不是很合理", s);
        }
        _ => println!("{}", String::from("正常"))
    }
    // 上述match 语句只有一个分支可以使用 if let 语句替代
    if let Sex::unkonw(s) = uokonw {
        println!("{} 不是很合理", s);
    } else {
        println!("{}", String::from("正常"));
    } 
}

enum Sex {
    man,
    woman,
    unkonw(String)
}

struct UserStruct {
    username:String,
    sex:u8,
    address:String,
    active:bool
}

impl UserStruct {
    fn set_sex(&mut self, sex: Sex) {
        // 使用 match Sex 枚举值 作为开头
        match sex {
            // 枚举值为 Sex::man 时 执行 => 后续代码
            Sex::man => self.sex = 1,
            // 枚举值为 Sex::woman 时 执行 => 后续代码
            Sex::woman => self.sex = 2,
            // 枚举值为 Sex::unkonw 时 执行 => 后续代码
            // 使用 sex_name 接收 sex 为 Sex::unkonw 时绑定的数据
            Sex::unkonw(sex_name) => {
                // 接收 sex_name 进行使用
                println!("{}", sex_name);
                self.sex = 3
            }
        }
    }

    fn print(&self) {
        println!("{}", if self.sex == 1 { "男" } else {if self.sex == 2 { "女" } else {"不确定"}});
    }
}

impl UserStruct {
    fn new(username: String, sex: Sex, address: String, active: bool) -> Self {
        Self{
            username,
            address,
            active,
            sex:match sex {
                // 表达式结果为 1 表示 符合这条条件的返回值为 1
                Sex::man => 1,
                Sex::woman => 2,
                Sex::unkonw(sex_name) => {
                    println!("{}", sex_name);
                    3
                }
            }
        }
    }
}