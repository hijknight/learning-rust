struct Point {
    x: u32,
    y: u32,
}


fn main() {

    // complex destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10});
}





// enums
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);
//
//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.");
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {x} and in the y direction {y}");
//         }
//         Message::Write(text) => {
//             println!("Text message: {text}");
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {r}, green {g}, and blue {b}")
//         }
//     }
// }


    // destructuring with patterns

    // let point = Point {
    //     x: 50,
    //     y: 25,
    // };
    //
    // let Point { x: a, y: b } = point;

    // assert_eq!(0, a);
    // assert_eq!(7, b);
    // more simplified = {
    //    let Point { x, y } = p;
//     assert_eq!(0, x);
//     assert_eq!(7, y);
    // }
    // ranges with char values

    // let x = 'c';
    //
    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }



    // ranges
    // same as 1 | 2 | 3 | 4 | 5
    // let x = 5;
    //
    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }


    // multiple patterns

    // let x = 1;
    //
    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("Anything"),
    // }



    // shadowing

    // let x = None;
    // let y = 10;
    //
    // match x {
    //     Some(50) => println!("50"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {x:?}"),
    // }
    //
    // println!("At the end: x = {x:?} and y = {y}");

    // matching

    // let x = 1;
    //
    // match x {
    //     1 => println!("One!"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }
