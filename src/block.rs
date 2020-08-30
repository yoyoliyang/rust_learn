fn main() {
    let x = 10;
    // block作用区域
    {
        let y: i32 = 20;
        println!("{}", y);
    }
    // println!("{},{}", x, y);
}
