#[allow(dead_code)]
#[allow(unused_variables)]

fn scope_and_shadowing(){
    let a = 123;

    // curly brases creates a scope
    // the variable inside these curly brases
    // will be only accessible inside this scope
    {
        let b = 456;
        println!("inside, b = {}", b);

        // shadowing example
        // This a and outside a are entirely different variables
        // but this a shadows the outside a varaible
        // if we disable this variable declaration here
        // we'll get the value from outside a variable
        let a = 777;
        // the variable a is available throughout the scope
        // It also exists in thsi inner scope as well
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);

    // If you try to compile the following line 
    // the code will throw an error like:  cannot find value `b` in this scope
    // because the variable b is not accessible outside the scope
    // println!("outside, b = {}", b);
}

fn main() {
    scope_and_shadowing();
}
