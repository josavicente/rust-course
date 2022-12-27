fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");
}
