fn main() {
    // if , else if, else
    // if 给定的必须是一个布尔值bool
    // if 是一个表达式，所以可以在let中使用
    let x: i64 = 20;

    if x > 30 {
        println!("number is less than 30");
    }

    if x == 45 {
        println!("ok");
    } else {
        println!("{}", x);
    }

    let number = 3;
    if number != 0 {
        // if number { 这样表述是错误的，不同于python或js
        println!("is 3");
    }

    let number = 10;
    if number == 5 {
        println!("is 5");
    } else if number == 3 {
        println!("is 3");
    } else {
        println!("is 10");
    }

    let condition = true;
    let number = if condition { 100 } else { 101 }; // if表达式的返回值作为变量数据
    println!("{}", number);
    // let string = if condition { "hello" } else { 101 }; // if表达式返回值必须为相同类型
}
