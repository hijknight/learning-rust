//
// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }
//
//
//
//
//
//
//
// fn main() {
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
//
//     let loopback = IpAddrKind::V6(String::from("::1"));
//
//
//
//
// }


// #[derive(Debug)]
// //
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let m = Message::Write(String::from("Hello"));
//
//     m.call();
// }
//
//
//
// impl Message {
//     fn call(&self) {
//         println!("{self:#?}");
//     }
//
//
// }


use std::io;
#[allow(dead_code)]
fn main() {
    let some_number: Option<i32> = Some(50);

    let some_char = Some('e');

    let mut some_string: Option<String> = Some(String::new());

    let absent_number: Option<i32> = None;


    let some_number: i32 = some_number.unwrap();
    let x: u32 = 5;
    let y: u32 = Some(5).unwrap();

    let sum = x + y;







}