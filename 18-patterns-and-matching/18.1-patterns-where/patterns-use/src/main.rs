fn main() {

    // fn parameters
    fn foo(x: i32) {
        // code goes here
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: {x}, {y}");
    }

    /*
    fn main() {
        // point is a tuple: (i32. i32)
        let point = (3, 5);
        print_coordinates(point)
    }
    */

    // let statements
    // let PATTERN = EXPRESSION;
    // let x = 5;
    // destructuring
    // let (x, y, z) = (1, 2, 3);
    // will panic -> let (x, y) = (1, 2, 3)


    // for loops

    // let v = vec!["a", "b", "c"];
    //
    // for (i, v) in v.iter().enumerate() {
    //     println!("{v} is at index {i}");
    // }

    // while let patterns

    // let mut stack = Vec::new();
    //
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // while let Some(top) = stack.pop() {
    //     println!("{top}");
    // }




    // if let patterns

    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();
    //
    // if let Some(color) = favorite_color {
    //     println!("Using you favorite color, {color}, as background");
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using Purple as the background color.");
    //     } else {
    //         println!("Using Orange as the background color.");
    //     }
    // } else {
    //     println!("Using blue as the background color.")
    // }
}
