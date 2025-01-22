use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1(x: u32) -> u32 { x + 1 }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1 };
    let add_one_v4 = |x: u32| x + 1;

    let example_closure = |x| x;

    let s = example_closure(String::from("Hello")); // infers type String
    let n = example_closure(String::from("Gotcha!")); // when an integer is passed in, it panics, because the String type has already been infered
}
