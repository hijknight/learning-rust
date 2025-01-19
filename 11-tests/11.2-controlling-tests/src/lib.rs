// run `cargo test -- --ignored
// ignoring tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//
//     #[test]
//     #[ignore]
//     fn expensive_test() {
//         // code that takes an hour to run
//     }
// }



pub fn add_two(a: usize) -> usize {
    a + 2
}



mod test;
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn add_two_and_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }
//
//     #[test]
//     fn add_three_and_two() {
//         let result = add_two(3);
//         assert_eq!(result, 5);
//     }
//
//     #[test]
//     fn one_hundred() {
//         let result = add_two(100);
//         assert_eq!(result, 102);
//     }
// }


//
// pub fn prints_and_returns_ten(a: i32) -> i32 {
//     println!("I got {a}");
//     10
// }
//
// #[cfg(test)]
// mod tests {
//
//
//     use super::*;
//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_ten(3);
//         assert_eq!(value, 10);
//     }
//
//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_ten(6);
//         assert_eq!(value, 5);
//     }
// }
