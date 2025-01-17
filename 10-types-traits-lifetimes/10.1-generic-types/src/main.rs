// structs
struct Point<T> {
    x: T,
    y: T,
}

struct Precise<T, U> {
    x: T,
    y: U,
}

// in impl

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// enums
// ex option and result enums:

// enum Option<T> {
//    Some(T),
//    None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {

    // let chars = vec!['w', 'a', 'y', 'z', 'Z', 'A'];
    // let largest_char = largest(&chars);
    //
    // println!("The largest character is {largest_char}");
    //
    // let integer = Point{ x: 10, y: 5 };
    // let float = Point{ x: 10.5, y: 5.5 };
    // // let wont_work = Point{ x: 5, y: 10.5 };
    // let will_work = Precise{ x: 5.5, y: 5 };

    let p = Point{ x: 10, y: 67 };
    println!("x value in point is {}", p.x());

    let d = Point{ x: 10.0, y: 15.5 };

    println!("Distance of point from origin is {}", d.distance_from_origin())
}

// fn largest_in_list(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// combine above into generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}