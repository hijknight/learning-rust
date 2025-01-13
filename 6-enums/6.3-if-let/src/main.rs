// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {max}"),
//         _ => (),
//     }
//
// }


//
// fn main() {
//     let config_max = Some(5u8);
//     if let Some(num) = config_max {
//         println!("The maximum value is configured to be {num}")
//     }
//
// }


#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn count_coins(coin: Coin) {
    let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("{count} count");
}



fn main() {



    let quarter: Coin = Coin::Quarter(UsState::Alabama);
    let penny: Coin = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;

    count_coins(quarter);




}
