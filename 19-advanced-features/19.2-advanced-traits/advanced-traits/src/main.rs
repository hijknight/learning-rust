




use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}


// <Type as Trait>::function(receiver_if_method, next_arg, ...);
// trait Pilot {
//     fn fly(&self);
// }
//
// trait Wizard {
//     fn fly(&self);
// }
//
// struct Human;
//
// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }
//
// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }
//
// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }
//
// fn main() {
//     let person = Human;
//     person.fly();
// }



//use std::ops::Add;

// struct Millimeters(u32);
// struct Meters(u32);
//
// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;
//
//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }


/*use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
*/