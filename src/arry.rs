// use std::str;

struct IPv4 {
    one: u8,
    two: u8,
    three: u8,
    four: u8,
}
fn main() {
    // array i32型,共5位
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //注意定义arry类型中的分号
    let numbers1 = [2; 10]; // 定义10个包含2的array

    // println!("number {}", numbers[1]);

    for i in numbers.iter() {
        println!("{}", i);
    }
    for i in numbers1.iter() {
        println!("{}", i);
    }

    let x = 10;
    println!("{}", x);

    // 不会清除s
    let s = String::from("hello");
    println!("{}", s);
    println!("{}", s);

    let ip1 = IPv4 {
        one: 192,
        two: 168,
        three: 0,
        four: 254,
    };

    println!(
        "你的IP地址是：{}.{}.{}.{}",
        ip1.one, ip1.two, ip1.three, ip1.four
    );
}
