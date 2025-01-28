mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist() {}
    }
}

// absolute path to module (idiomatic)
pub use crate::front_of_house::hosting;


pub fn eat_at_restaurant() {
    // show that function add_to_waitlist is not local, by calling it through the hosting module for functions
    hosting::add_to_waitlist();
}

// use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {}
//
// fn function2() -> io::Result<()>{}

// or

// use std::fmt::Result;
// use std::io::Result as IoResult; // using as keyword
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

//
