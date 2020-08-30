fn main() {
    let tup1 = (1, 2, 3, 4);
    let tup1: (u32, f64, bool, i32) = (12, 3.14, true, -10);
    let (a, b, c, d) = tup1;
    println!("{}, {}, {}, {}", a, b, c, d);
}

// 函数中元组作为参数的用法
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
