use std::fs;
use std::io::{self, Read};

fn main() {

    // let greeting_result = File::open("username.txt");
    //
    //
    //  let greeting_file = match greeting_result {
    //      Ok(file) => file,
    //      Err(error) => match error.kind() {
    //          ErrorKind::NotFound => match File::create("hello.txt") {
    //              Ok(fc) => fc,
    //              Err(e) => panic!("Error occurred. {e:?}"),
    //          },
    //          other_error => {
    //              panic!("Problem opening the file: {other_error:?}")
    //          }
    //      }
    //  };
    //
    // let greeting = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("file could not be created");
    //         })
    //     } else {
    //         panic!("A problem occurred when opening file: {error:?}")
    //     }
    // });


   // let greeting_file = File::open("hello.txt").expect("No file found");

    let mut username = read_username_from_file("username.txt").unwrap();


    println!("{username:?}")



}

// verbose
// fn read_username_to_file(file: &str) -> Result<String, io::Error> {
//     let username_file_result = File::open(file);
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
//
// }

// more efficient
// fn read_username_from_file(file: &str) -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open(file)?.read_to_string(&mut username)?;
//
//     Ok(username)
//
// }


//most efficient
fn read_username_from_file(file: &str) -> Result<String, io::Error> {
    fs::read_to_string(file)
}
