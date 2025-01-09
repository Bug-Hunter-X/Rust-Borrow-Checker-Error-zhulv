fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y = 10; // Modifying x through y
    println!("x = {}", x); // x is now 10

    // The following line will cause a compile-time error
    // because we are trying to modify x while z is an active immutable reference
    // *z = 20; 
}