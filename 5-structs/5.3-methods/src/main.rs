// methods


#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn nonzero(&self) -> bool {
        return dbg!(self.width > 0 && self.height > 0)

    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }


    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 60,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 70,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 30,
    };

    let square = Rectangle::square(50);

    println!("{square:#?}");

    println!("Can rect0 hold rect1? {}", rect0.can_hold(&rect1));
    println!("Can rect0 hold rect2? {}", rect0.can_hold(&rect2));
    println!("Can rect0 hold rect3? {}", rect0.can_hold(&rect3));


    // println!(
    //     "The area of the rectangle is {} and is nonzero; it is {}.",
    //     rect1.area(),
    //     rect1.nonzero()
    // );

}

