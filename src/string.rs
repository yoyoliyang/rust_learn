fn main() {
    let mut my_string = String::from("你 好 世界！");

    println!("字符串长度： {}", my_string.len());
    println!("字符串是空的吗？ {}", my_string.is_empty());

    my_string.push_str("hello world!");

    for t in my_string.split_whitespace() {
        println!("{}", t);
    }

    let s = &mut my_string[0..3]; // 注意一个汉字是4个字节
    println!("{}", s);

    let mut str1 = "hello,world".to_string();
    str1.push_str("你好世界");
    str1 = str1 + "liyang";
    let str2 = "o";
    let str3 = "c";
    let str1 = format!("{}-{}", str2, str3);
    println!("{}", str1);

    //遍历字符串
    let str1 = "hello,world!你好世界！";
    for i in str1.chars() {
        println!("{}", i);
    }
    //遍历字节
    for i in str1.bytes() {
        println!("{}", i);
    }

    //整型实现了Display的特性
    println!("{}", 10.to_string());
}
