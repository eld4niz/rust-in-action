fn anonymous_for_loop() {
    for _ in 0..10 {
        println!("idk man");
    }
}

fn collection_for_loop() {
    let arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        let item = arr[i];
        println!("item: {}", item);
    }
}

fn continue_break() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        if i == 7 {
            break;
        }
        println!("i: {}", i);
    }
}

fn switch_key_alternative() {
    let _needle = 42; // redundant variable(_) to avoid warning
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let res = match item {
            42 | 132 => "hit", // multiple values
            _ => "miss",       // default case
        };

        if res == "hit" {
            println!("{}: {}", item, res);
        }
    }
}

fn main() {
    anonymous_for_loop();
    collection_for_loop();
    continue_break();
    switch_key_alternative();
}
