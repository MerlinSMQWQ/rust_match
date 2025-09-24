// rust提供了一个非常强大的控制流运算符match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    let nickel = Coin::Nickel;
    println!("{}", value_in_cents(nickel));
    let dime = Coin::Dime;
    println!("{}", value_in_cents(dime));
    let quarter = Coin::Quarter;
    println!("{}", value_in_cents(quarter));
}
