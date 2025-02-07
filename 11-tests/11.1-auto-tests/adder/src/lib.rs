
// should panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess should be between 1 and 100, got {value}")
        }

        Guess{value}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
pub fn greeting(name: &str) -> String {
    // format!("Hello, {name}")
    String::from("Hello")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, result was `{result}`"
        );
    }
}


// pub fn add_two(a: usize) -> usize {
//     a + 2
// }
//
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4)
//     }
// }





// struct Rectangle {
//     width: u32,
//     height: i32,
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//
//         assert!(larger.can_hold(&smaller));
//     }
//
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(!smaller.can_hold(&larger)); // negated with '!'
//     }
//
//
//
//     // #[test]
//     // fn exploration() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
//     //
//     // #[test]
//     // fn another() {
//     //     panic!("Make this test fail");
//     // }
//
//
// }
