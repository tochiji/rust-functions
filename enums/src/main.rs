#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let c: Coin = Coin::Penny;
    let n: Coin = Coin::Nickel;
    let d: Coin = Coin::Dime;
    let q: Coin = Coin::Quarter;
    let cm = value_in_cents(c);
    let nm = value_in_cents(n);
    let dm = value_in_cents(d);
    let qm = value_in_cents(q);
    println!("{}", cm);
    println!("{}", nm);
    println!("{}", dm);
    println!("{}", qm);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?}", six, none);

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three!"),
        None => (),
        _ => (),
    }

    if let Some(9) = some_u8_value {
        println!("three, too!");
    } else {
        println!("Go to hell");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luckey penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}
