## Numbers

### script 2.2(defining_variables) - adding integers using variables and declaring types

`defining_variables/main.rs`: **line 3** - declares a variable `b` of type `i32` and assigns it the value 10.

`defining_variables/main.rs`: **line 4-5** - Rust's numeric literals can include types annotations to specify the type of the literal. These increase readability but are insignificant to the compiler.

`defining_variables/main.rs`: **line 11** - you can see that Rust’s syntax for defining functions is similar to those programming languages that use explicit type declarations. Commas delimit parameters, and type declarations follow variable names. The dagger (->) or thin arrow syntax indicates the return type.

### script 3.3(numbers) - comparing numbers

Rust’s type safety requirements prevent comparisons between types. For example, this
code does not compile:

```rust
fn main() {
    let a: i32 = 10;
    let b: u16 = 20;
    if a < b {
        println!("a is less than b");
    }
}
```
**Warning:** Using type casts carelessly will cause your program to behave unexpectedly. For example, the expression `300_i32` as `i8` returns 44.

- The binary representation of `300_i32` is 00000000 00000000 00000001 `00101100`.
- When truncated to an 8-bit integer (`i8`), we take the least significant 8 bits: 00101100.
- The decimal value of 00101100 is 44.

### Floating-point hazards

Floating-point types (f32 and f64, for example) can cause serious errors for the unwary. There are (at least) two reasons for this:

- These often approximate the numbers that they’re representing. Floating-point types are implemented in base 2, but we often want to calculate numbers in base 10
- These can represent values that have unintuitive semantics. Unlike integers, floating-point types have some values that do not play well together(by design).

To prevent these hazards, these are two guidelines to follow:
- Avoid testing floating-point numbers for equality.
- Be wary when results may be mathematically undefined.

```rust
fn main() {
    assert!(0.1 + 0.2 == 0.3);
}
```

### script 2.3.4(complex-numbers) - complex numbers in Rust 

Rust’s standard library is comparatively slim. It excludes numeric types that are often available within other languages. These include: 
- Many mathematical objects for working with rational numbers and complex numbers
- Arbitrary size integers and arbitrary precision floating-point numbers for working with very large or very small numbers
- Fixed-point decimal numbers for working with currencies

To access these specialized numeric types, we can use the num crate. 

Specific points for complex numbers script:

- The `use` keyword is used to bring the `num` type into scope and the namespace operator (`::`) restricts what’s imported
- Rust does not have constructors; instead, every type has a literal form.You can initialize types by using the type name (Complex) 
  and assigning their fields (re, im) values within curly braces (`{ }`).
- Many types implement a `new()` method for simplicity. This convention, however, is not part of the Rust language.
-  `num::complex::Complex` type has two fields: `re` represents the real part, and im represents the imaginary part. 
  Both are accessible with the dot operator.

### External cargo commands by cargo-edit

To install cargo-edit, run the following command:
```bash
cargo install cargo-edit
```

To add a dependency to our project, run the following command:
```bash
cargo add num
```

To remove a dependency from our project, run the following command:
```bash
cargo rm num
```

## Flow control

### for loop

When you want to reuse container later in your program, use a reference. Rust assumes that container is no longer needed. 
To add a reference to the container, prefix it with an ampersand (&) as this example shows:

```rust
for item in &container {
    // ...
}
```

If you need to modify each item during the loop, you can use a mutable reference by including the mut keyword:

```rust
for item in &mut container {
    // ...
}
```

### while loop

The while loop proceeds as long as a condition holds. The condition, formally known as a predicate, can be any expression that evaluates to true or false.

example:
```rust
let mut samples = vec![];

while samples.len() < 10 {
    let sample = take_sample();
    
    if is_outlier(sample) {
        continue;
    }
    
    samples.push(sample);
}
```

### Rust's own loop(more preferred than for/while loop)

Rust contains a loop keyword for providing more control than for and while. loop executes a code block again and again, never stopping for a tea (or coffee) break.
loop continues to execute until a break keyword is encountered or the program is terminated from the outside.

example:

```rust
loop {
    let requester, request = accept_request();
    let result = process_request(request);
    send_response(requester, result);
}
```

### conditional branching

Rust has no concept of “truthy” or “falsey” types. Other languages allow special values such as 0 or 
an empty string to stand in for false and for other values to represent true, but Rust doesn’t allow this. The only value that can be used for true is true, and for false, use false.

### match: Type-aware pattern matching

match offers a sophisticated and concise syntax for testing multiple possible values. Some examples are:
- Inclusive ranges (10 ..= 20) to match any value within the range.
- A Boolean OR (|) will match when either side matches.
- A wildcard (_) will match any value.

example:
```rust
match value {
    0 => println!("zero"),
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("many"),
}
```
`match` is analogous to the switch keyword in other languages. Unlike C’s `switch`, however, match guarantees that all possible options for a type are explicitly handled.

## Defining Functions

Rust’s functions require that you specify your parameter’s types and the function’s return type. This is the foundational knowledge that we’ll need for the majority of our work with Rust.

### Using references
A reference is a value that stands in place for another value. For example, imagine that variable *a* is a large array that is costly to duplicate. In some sense, a reference *r* is a cheap copy of *a*. But instead of creating a duplicate, the program stores a’s address in memory. When the data from a is required, *r* can be dereferenced to make a available.

### Generic Functions
Capital letters in place of a type indicate a generic type. Conventionally, the variables T, U, and V are used as placeholder values, but this is arbitrary. E is often used to denote an error type.

## List of Things

### Arrays
[T; n] describes an array’s type, where T is the elements’ type and n is a non-negative integer. [f32; 12] denotes an array of 12 32-bit floating-point numbers. 
It’s easy to get confused with slices [T], which do not have a compile-time length.

Rust maintains its focus on safety. Array indexing is bounds checked. Requesting an
item that’s out of bounds crashes (panics in Rust terminology) the program rather
than returning erroneous data.

### Slices
Slices are dynamically sized array-like objects. The term dynamically sized means that
their size is not known at compile time. Yet, like arrays, these don’t expand or contract.

The lack of compile-time knowledge explains the distinction in the type signature between an array ([T; n]) and a slice ([T]).


