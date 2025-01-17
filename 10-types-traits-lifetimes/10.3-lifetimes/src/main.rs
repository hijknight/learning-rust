// fn main() {
//                           // lifetime annotations
//     let r;          // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;   // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+

// fn main() {
//     let x = 5;             // ----------+-- 'b
//                                 //           |
//     let r = &x;           // --+-- 'a  |
//                                 //   |       |
//     println!("r: {r}");         //   |       |
//                                 // --+       |
// }                               // ----------+

// generic lifetimes in functions

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);


}

// fn longest(s1: &str, s2: &str) -> &str { // does not work
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }


fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}