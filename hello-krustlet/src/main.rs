use std::{thread, time};
fn main() {
    loop {
        println!("Hello krustlet! 🦀");
        let d= time::Duration::from_millis(750);
        thread::sleep(d);
    } 
}
