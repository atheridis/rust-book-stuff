enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    California,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let my_coin: Coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(my_coin));

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // This thing loopback into another place

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}

fn route(ip_kind: IpAddr) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State qurter from {:?}!", state);
            25
        }
    }
}
