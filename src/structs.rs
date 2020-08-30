// struct , 首字母应该大写

struct Color {
    red: u8, // 8位无符号整型
    green: u8,
    blue: u8,
}

#[derive(Debug)] // 注意一定将该注解放到结构体上方
struct Rectangle {
    width: u32,
    height: u32,
}

struct Color1(u8, u8, u8); // 元组结构体

fn main() {
    let mut rectangle1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rectangle1 is {:#?}", rectangle1);

    let bg = Color {
        red: 255,
        green: 20,
        blue: 14,
    };

    let bg0 = Color1(255, 255, 255);

    let bg1 = Color { red: bg0.0, ..bg }; // 使用bg0的值，之后使用bg剩余的值

    let mut fg = Color1(244, 30, 2);
    fg.0 = 255;

    println!("{}, {}", bg.red, bg1.red);

    println!("the area of the rectangle1 is :{}", area(&mut rectangle1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
