fn main() {
    let a = 8; /* type inferred variable */
    let b: i32 = 9; /* type annotated variable */
    let c = 30i32; /* type annotated variable */
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}

fn add(x: i32, y: i32) -> i32 {
    x + y /* no semicolon means return */
}
