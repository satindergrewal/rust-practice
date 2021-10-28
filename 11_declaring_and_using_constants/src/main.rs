#[allow(dead_code)]
#[allow(unused_variables)]

const MEANING_OF_LIFE:u8 = 42; // no fixed memory address

static mut Z:i32 = 123;

fn main() {
    
    // In most cases if you need a constant like Pi
    // you must use const instead of static
    println!("Meaning of Life: {}", MEANING_OF_LIFE);

    // Other processes might be using same global variable
    // so it's not ideal to use mutable global variable.
    // But if you has to use a mutable global variable
    // you must declare "unsafe" scope for it
    unsafe
    {
        // You can also assign different value 
        // to global mutable variable
        Z = 777;
        println!("Z: {}", Z);
    }
}
