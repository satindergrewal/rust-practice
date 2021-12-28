#[allow(dead_code)]
#[allow(unused_variables)]

pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000
    {
        x *= 2;

        // matching this if statement the next part of code gets skipped
        // and loop continues for next conditions
        if x == 64 { continue; }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);

        // using break statement breaks the loop and exits from the loop condition
        if y == 1<<10 { break; }
    }
}

fn main() {
    while_and_loop();
}
