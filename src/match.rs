fn main() {
    let number: i32 = 110;

    match number {
        1 => println!("It is one!"),
        2 => println!("It is two!"),
        // 10 | 11 => println!("10 or 11"),
        10..=20 => println!("10 ~ 20"),
        _ => println!("不符合"),
    }

    let name = "ocean";

    match name {
        "li" => println!("your name is {}", name),
        "ocean" => println!("your name is {}", name),
        _ => println!("nothing"),
    }
}
