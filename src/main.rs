enum IpAddrKind{
    V4 (u8,u8,u8,u8),
    V6 (String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum Message{
    Quit,
    Move {x:u32, y:u32},
    Write (String),
    ChangeColor (i32, i32, i32)
}

impl Message {
    fn some_function(){
        println!("RUST PROG LANG!!");
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhoust = IpAddrKind::V4(127,0,0,1);
    Message::some_function();
    let x: u32 = 5;
    let y:Option<u32> = Some(5);
    let sum = x + y.unwrap_or(0);
    println!("{}",sum);
    let coin_val = coin_type(Coin::Penny);
    println!("The coint value is: {}", coin_val);
}

fn coin_type(value: Coin) -> u8{
    match value{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}