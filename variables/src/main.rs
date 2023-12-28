fn main() {
    /*
    let x = 5;
    println!("x = {x}");
    x = 6; // compile error for reassigning an immutable variable
    println!("x = {x}");
    */
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
