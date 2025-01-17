

use crate::lib::*;

fn main() {
    let tweet = Tweet {
        username: String::from("hijknight"),
        content: String::from("Do you even know about rust?"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet from {}", tweet.summarize())
}

mod lib {
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    pub trait Summary {
        //default implementation
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }


    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
