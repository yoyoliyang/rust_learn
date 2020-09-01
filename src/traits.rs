struct Person {
    name: String,
    age: u8,
}
struct Dog {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    fn speak(&self) -> String;
    fn can_speak(&self) -> bool;
}
// 把trait定义的方法传递给struct中
impl HasVoiceBox for Person {
    fn speak(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

impl HasVoiceBox for Dog {
    fn speak(&self) -> String {
        format!("I am {}", self.name)
    }
    fn can_speak(&self) -> bool {
        false
    }
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 15,
    };

    let tom = Dog {
        name: "Tom".to_string(),
        age: 7,
    };

    println!(
        "{}\n{} years old.\nCap speak?{}",
        tom.speak(),
        tom.age,
        tom.can_speak()
    );

    let mut dash = "_".to_string();
    let dash = {
        for _ in 0..20 {
            dash += "_"
        }
        dash
    };
    println!("{}", dash);

    println!("{}\nCan speak?{}", person.speak(), person.can_speak());
}
