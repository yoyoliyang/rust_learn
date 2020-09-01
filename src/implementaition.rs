struct Cav {
    width: u32,
    heigth: u32,
}

struct Person {
    name: String,
    age: u8,
}

// 对结构体Cav的默认特性
impl Cav {
    // implementation 类似python中的类
    fn print_wh(&self) {
        println!("{} x {}", self.width, self.heigth);
    }

    fn return_w_plus_h(&self) -> u32 {
        // let p = self.width + self.heigth;
        return self.width + self.heigth;
    }

    fn return_w_eq_h(&self) -> bool {
        return self.width == self.heigth;
    }
}

impl ToString for Person {
    // 将ToString自带的方法to_string重新定义！
    fn to_string(&self) -> String {
        return format!("my name is {} and I am {}", self.name, self.age);
    }
}
fn main() {
    let a = Cav {
        width: 10,
        heigth: 20,
    };

    a.print_wh();
    println!("{}", a.return_w_plus_h());
    println!("{}", a.return_w_eq_h());

    let xiaoming = Person {
        name: String::from("xiaoming"),
        age: 32,
    };

    println!("{}", xiaoming.to_string());

    println!("{}", "hello,world".to_string());
}
