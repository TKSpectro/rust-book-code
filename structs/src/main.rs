struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}

fn main() {
    let rect = Rectangle {
        height: 2,
        width: 4,
    };

    let _area = area(&rect);

    println!("Rectangle area: {}", rect.area());
}
