fn main() {
    let mut x = 5;
    { // Creating a new scope to limit the lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    }

    let z = &x;  // Now it's safe to create an immutable reference
    println!("x = {}", x); // x is now 10
    println!("z = {}", z); // z is 10
} 