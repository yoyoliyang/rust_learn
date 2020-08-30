fn main() {
    // replace
    {
        let string = String::from("Hello world!");
        println!("替换：{}", string.replace("H", "Y"));
    }

    // lines
    {
        // let string = String::from("Hello\nworld\n");
        let string = "hello\nworld\n";
        for line in string.lines() {
            println!("[ {} ]", line);
        }
    }

    {
        // Split;
        let my_string = String::from("Hello+world+li+yang");
        let tokens: Vec<&str> = my_string.split("+").collect();

        for t in tokens.iter() {
            println!("{}", t);
        }
    }

    {
        //Trim
        let my_string = String::from("  Hello world liyang     \n\r");
        println!("{}", my_string);
        println!("{}", my_string.trim());
    }

    {
        //chars
        let string = String::from("你好，Hello world!");

        match string.chars().nth(1) {
            Some(c) => println!("{}", c),
            None => println!("None"),
        }
    }
}
