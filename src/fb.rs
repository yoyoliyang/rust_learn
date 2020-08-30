fn main() {
    // 斐波那契数列计算
    let mut a: i32 = 0;
    let mut b: i32 = 1;

    let mut n = 10;

    while n > 0 {
        // 方法一
        // let c = a + b;
        // a = b;
        // b = c;
        // n -= 1;
        // print!("{},", c);
        // 方法二
        b = a + b;
        a = b - a;
        n -= 1;
        print!("{},", b);
    }
}
