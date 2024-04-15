## script 2.2(defining_variables) - adding integers using variables and declaring types

`defining_variables/main.rs`: **line 3** - declares a variable `b` of type `i32` and assigns it the value 10.

`defining_variables/main.rs`: **line 4-5** - Rust's numeric literals can include types annotations to specify the type of the literal. These increase readability but are insignificant to the compiler.

`defining_variables/main.rs`: **line 11** - you can see that Rust’s syntax for defining functions is similar to those programming languages that use explicit type declarations. Commas delimit parameters, and type declarations follow variable names. The dagger (->) or thin arrow syntax indicates the return type.

## script 2.3.3 - comparing numbers

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

- The binary representation of `300_i32` is 00000000 00000000 00000001 **00101100**.
- When truncated to an 8-bit integer (`i8`), we take the least significant 8 bits: 00101100.
- The decimal value of 00101100 is 44.

## Floating-point hazards

Floating-point types (f32 and f64, for example) can cause serious errors for the unwary. There are (at least) two reasons for this:

- These often approximate the numbers that they’re representing. Floating-point types are implemented in base 2, but we often want to calculate numbers in base 10
- These can represent values that have unintuitive semantics. Unlike integers, floating-point types have some values that do not play well together(by design).

To prevent these hazards, these are two guidelines to follow:
-  
