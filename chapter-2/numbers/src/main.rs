use std::convert::TryInto;

fn numeric_literals() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("twenty + twenty_one + twenty_two = {}", addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:02}", forty_twos[0]);
}

fn using_base() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {}, {}, {}", three, thirty, three_hundred);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred); // binary
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred); // octal
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred); // hexadecimal
}

fn try_into_example() {
    let a: i32 = 300;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("hundred is lett than three hundred");
    }
}

fn main() {
    numeric_literals();
    using_base();
    try_into_example();
}