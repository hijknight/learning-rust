// fn main() {
//     let number: u32 = rand::thread_rng().gen_range(1..10);
//     println!("number is {number}");
//     if number < 5 {
//         println!("number is less than 5");
//
//     } else {
//         println!("number is greater than 5");
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {}", result);
// }

// fn main() {
//     let mut count: u8 = 0;
//
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining: u8 = 10;
//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1
//         }
//         count += 1;
//     }
//     println!("End count = {}", count);
// }

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!")
// }
//
//
//
// While loops iterating through an array
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index: usize = 0;
//
//     while index < 5 {
//         println!("the value at index {} is {}", index, a[index]);
//         index += 1
//     }
// }

// fn main() {
//     let a: [i32; 5] = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     for i in a {
//         index += 1;
//         println!("the value at {index} is {i}")
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
