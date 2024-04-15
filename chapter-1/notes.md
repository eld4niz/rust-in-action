# Notes on current chapter

## script 1.2 - printing out csv type data

*Method syntax* — Although Rust is not object-oriented as it **does not support inheritance**, it carries over this feature of object-oriented language.

*Implicit return* - Rust provides a return keyword, but it’s usually omitted. Rust is an *expression-based language*

`csv-processing/src/main.rs` **line 19** - A vector in Rust is an array-like data structure with dynamic size, meaning it can grow or shrin as needed. 
On the other hand, an array hasa fixed size and cannot be dynamically resized.

`csv-processing/src/main.rs` **line 22-28** - The `println!` macro prints its arguments to standard out (stdout), whereas `eprintln!` prints to standard error (stderr)

`csv-processing/src/main.rs` **line 27** - Attempts to parse fields[1] as a 32-bit floating-point number and, if that is successful, then assign the number to the length variable.

## script 1.3 - goal of rust: safety

**dangling-pointers** — Live references to data that has become invalid over the course of the program


```rust
#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    println!("{:?}", grains);
}
```

**data-race** - The inability to determine how a program will behave from run to run because external factors change

```rust
use std::thread;

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });

    println!("{}", data);
}
```

**buffer overflow** - An attempt to access the 12th element of an array with only 6 elements

```rust
fn main() {
  let fruit = vec!['🥝', '🍌', '🍇'];

  let buffer_overflow = fruit[4];

  assert_eq!(buffer_overflow, '🍉')
}
```

**iterator-invalidation** - An issue caused by something that is iterated over after being altered midway through

```rust 
fn main() {
  let mut letters = vec![
      "a", "b", "c"
  ];

  for letter in letters {
      println!("{}", letter);
      letters.push(letter.clone());
  }
}
```


