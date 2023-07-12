pub fn print_type(s:&str) {
    println!("{}", s);
}

pub struct Car {
    number:String,
    color:Color,
    brand:String
}

impl Car {
    pub fn new (number:String, color:Color, brand:String) -> Self {
        Self{
            number,
            color,
            brand
        }
    }

    pub fn get_number (&self) -> String {
        self.number.clone()
    }

    pub fn get_color (&self) -> Color {
        self.color.clone()
    }

    pub fn get_brand (&self) -> String {
        self.brand.clone()
    }

    pub fn set_number (&mut self, number:&String) {
        self.number = number.to_string();
    }

    pub fn set_color (&mut self, color:Color) {
        self.color = color;
    }

    pub fn set_brand (&mut self, brand:&String) {
        self.brand = brand.to_string();
    }
    
}

#[derive(Debug, Clone)]
pub enum Color {
    White,
    Red,
    Black,
    Bule,
    Green
}