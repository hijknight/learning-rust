// fn main() {
//
//
//     let user1 = User {
//         active: true,
//         username: String::from("harry"),
//         email: String::from("harry@icloud.com"),
//         sign_in_count: 1,
//     };
//
//
//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@icloud.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };
//
//     let user2 = User {
//         email: String::from("hijknight@icloud.com"),
//         ..user1
//     };
//
//
//
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }


// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
//
// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     let first_value = black.0;
//     println!("{first_value}");
// }

// struct AlwaysEqual;
//
// fn main() {
//     let subject = AlwaysEqual;
// }

fn main() {
    println!("5.1 - structs!");
}
