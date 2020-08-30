use std::process::Command;

fn main() {
    let mut cmd = Command::new("ls");
    cmd.arg("-l");

    match cmd.output() {
        Ok(o) => unsafe {
            println!("{}", String::from_utf8_unchecked(o.stdout));
        },

        Err(e) => {
            println!("Error {}", e);
        }
    }
}
