fn main() {
    // 只有在实现了Display特性的数据上才能使用println
    println!("{}", 3); //ok
    println!("{}", 3.to_string()); //ok
    let s = "hello";
    println!("{}", s); // ok
}
