// enum IpAddressKind {
//     V4, V6
// }

// we can put data directly inside the enum variant
#[derive(Debug)]
enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[allow(dead_code)]
struct IpAddress {
    kind: IpAddressKind,
    address:String
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move {x: i32, y: i32},
    Write(String),
    _ChangeColor(i32,i32,i32,i32)
}

// just like structs, we can define methods and associated functions on our enum type
impl Message {
    fn print_message() {
        println!("{:?}", Message::Write(String::from("Hey from enum implementation!")));
    }   
}

// enum Coin {
//     Penny, Nickel, Dime, Quarter
// }

// fn value_in_cents (coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25
//     }
// }

pub fn run() {
    println!("hello enums");
    let _four = IpAddressKind::V4;
    let _six = IpAddressKind::V6;

    // with simple enum
    // let localhost = IpAddress {
    //     kind: IpAddressKind::V4, 
    //     address: String::from("127.0.0.1")
    // };

    // with putting data inside the enum variant
    let localhost = IpAddressKind::V4(127,0,0,1);
    println!("{:?}", localhost);

    Message::print_message();

    let x:i8 = 5;
    let y: Option<i8> = Some(5); // even value `None` works here because we added unwrap_or() with a default value

    // ! cannot add `Option<i8>` to `i8`
    // let sum = x + y;

    // provide a default value for it to be valid
    let sum = x + y.unwrap_or(5);
    println!("add integer 5 with Option<5>.unwarp_or(default value): {}", sum);

    let five = Some(5);
    println!("{:?}", five);
    let six = plus_one(five);
    println!("{:?}", six);   

    // the if let expression

    let some_value = Some(3);
    // match some_value {
    //     Some(3) => println!("three"),
    //     _ => ()
    // }

    // the code above is ok, but we can make more concise with the if let expression
    if let Some(3) = some_value {
        println!("three")
    }
}

// combine Option enum with the match expression
fn plus_one(x:  Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}