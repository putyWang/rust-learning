fn main() {
    let screen = Screen {
        components: vec![
            // 使用实现了 Draw 的 selectBox 具体类型
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            // 使用实现了 Draw 的 Button 具体类型
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // 由于 String 未实现 Draw 因此该处报错
            // Box::new(String::from("Hi"))
        ],
    };

    screen.run();
}

// 定义通用行为 trait
pub trait Draw {
    fn draw(&self);
}

// 定义 Screen 结构体
// dyn Draw 表明 Box 需要的类型需要的是任何实现了 指定 trait 的替身
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 由于 Clone trait 是不安全的 因此下列声明无法通过编译；
// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>,
// }

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button:{:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox:{:?}", self);
    }
}