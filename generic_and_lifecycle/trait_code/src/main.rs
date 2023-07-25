
fn main() {
    let student = Tweet{
        username: String::from("张三"),
        content: String::from("是一名学生"),
        reply: true,
        retweet: true
    };

    // 调用为 Tweet 实现的 summarize 方法
    println!("{}", student.summarize());
    // 调用默认实现的 run 方法
    println!("{}", student.run());

    notify(&student);

    println!("{}", create_tweet().summarize());
}

// 使用 trait 关键字定义 Summary trait
trait Summary {
    // 定义没有方法体的 summarize 抽象方法
    fn summarize(&self) -> String;

    // 定义一个具有默认实现的方法 run
    fn run(&self) -> String {
        String::from("正在奔跑")
    }
}

trait IsTrue {
    fn is_true(&self) -> bool;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为 Tweet 结构 实现 Summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl IsTrue for Tweet {
    fn is_true(&self) -> bool {
        self.reply
    }
}

// 该实现会报错，因为 Vec 与 Display 都不是本地定义的 所以不能为 Vec 实现 Display
// impl<T> Display for Vec<T> {

// }

// 声明 notify 接收参数需要实现 Summary
// fn notify(item: &impl Summary) {
//     println!("{}", item.summarize());
// }

// 使用trait Bound 语法重写 notify方法
fn notify<T:Summary>(item: &T) {
    println!("{}", item.summarize());
}

// 声明 notify 接收参数需要实现 Summary 与 IsTrue
// fn display_item(item: &(impl Summary + IsTrue)) {
//     println!("{}", item.summarize());
// }

// 使用trait Bound 语法重写 display_item方法
fn display_item<T:Summary + IsTrue>(item: &T) {
    println!("{}", item.summarize());
}

// 以下两条声明 some_function 函数语句是等价的
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32 
// where
//     T: Display + Clone, 
//     U: Clone + Debug{

// }

// 以 impl Summary 作为返回值 表明 该函数返回值需要实现 Summary
fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}