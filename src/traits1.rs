struct Person {
    name: String,
    age: u8,
}
// 默认共享特性
trait HasVoiceBox {
    fn speak(&self) -> String {
        format!("Hello!")
    }
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    // 修改默认共享特性
    fn speak(&self) -> String {
        format!("I am {}", self.name)
    }
    fn can_speak(&self) -> bool {
        if self.age > 3 {
            return true;
        }
        return false;
    }
}
// impl HasVoiceBox for Dog {}
// 把trait定义的方法传递给struct中
// impl HasVoiceBox for Person {
// fn speak(&self) -> String {
// format!("Hello, my name is {}", self.name)
// }
//
// fn can_speak(&self) -> bool {
// if self.age > 0 {
// return true;
// }
// return false;
// }
// }
//
// impl HasVoiceBox for Dog {
// fn speak(&self) -> String {
// format!("I am {}", self.name)
// }
// fn can_speak(&self) -> bool {
// false
// }
// }
//
fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 15,
    };

    let mut dash = "_".to_string();
    let dash = {
        for _ in 0..20 {
            dash += "_"
        }
        dash
    };
    println!("{}", dash);

    println!("{}\nCan speak?{}", person.speak(), person.can_speak());
    notify(person);
}

fn notify(name: impl HasVoiceBox) {
    // 传递trait作为参数
    println!("{}", name.speak());
}
