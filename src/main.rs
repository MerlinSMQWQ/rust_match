// rust提供了一个非常强大的控制流运算符match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {    // 如果这里是多行的代码块，那就需要使用花括号括起来，有点像匿名函数
            println!("this is a penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}!", state);
            25
        },
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    let nickel = Coin::Nickel;
    println!("{}", value_in_cents(nickel));
    let dime = Coin::Dime;
    println!("{}", value_in_cents(dime));
    let quarter = Coin::Quarter(USState::Alabama);
    println!("{}", value_in_cents(quarter));
    let quarter = Coin::Quarter(USState::Alaska);
    println!("{}", value_in_cents(quarter));
}
