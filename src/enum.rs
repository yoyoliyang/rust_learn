// 枚举也可以使用ipml方法
#[derive(Debug)]
enum IpAddrKind {
    V4([u8; 4]),
    V6(String),
}

enum Coin {}
// impl IpAddrKind {
// fn call(&self) {
// self::V4

fn main() {
    let home = IpAddrKind::V4([192, 168, 1, 123]);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    // Option
    //
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap(); // option解包取值

    let five = Some(0u8);
    match five {
        Some(3) => println!("ok"),
        _ => (),
    }
    // let six = plus_one(five);
    // println!("{}", six.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
