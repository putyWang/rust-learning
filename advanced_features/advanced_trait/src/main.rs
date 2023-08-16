use std::fmt::{Display, Formatter, Result};

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 }.add(Point { x: 2, y: 3 }),
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // 默认调用 直接实现 Human 对象中的fly 方法
    person.fly();
    // 使用 trait 名::方法名(对象引用) 的方式调用实现对应trait的方法
    Wizard::fly(&person);
}

struct Counter{}

impl<T> AddCustomization<T> for Counter {
    // 由于 Iterator 拥有关联类型，因此该impl块中显示制定了 Output 类型为 u32
    type Output = Point;

    fn add(self, _: T) -> Point {
        Point{
            x:1,
            y:2
        }
    }
}

// 指定 泛型 默认类型为 Self
trait AddCustomization<Rhs=Self> {
    // 通过 type 关键字 声明 Iterator trait 关联类型占位符为 Output
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 未指定泛型类型 泛型为默认类型 Self 
impl AddCustomization for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// OutlinePrint trait 声明 Display 父 trait
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 为 Point 实现 OutlinePrint trait 必须同时实现其父 trait
impl OutlinePrint for Point {}

// 必须同时实现 trait 与 父 trait 
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}