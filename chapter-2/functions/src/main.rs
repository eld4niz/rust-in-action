use std::ops::Add;
use std::time::Duration;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn searching_integer() {
    let needle = 15;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140];

    for item in &haystack {
        if *item == needle {
            println!("{} is in the haystack!", item);
            return;
        }
    }
}

fn new_add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn new_add_apply() {
    let floats = new_add(1.2, 3.4);
    let ints = new_add(10, 20);

    let durations = new_add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}

fn main() {
    println!("{}", add(1, 2));
    searching_integer();
    new_add_apply();
}
