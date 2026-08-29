// Variables and Mutability
fn var() {
    let mut x = 5;
    println!("The value of x is {x}");

    // mut the value of x
    x = 6;
    println!("The value of x is {x}")
}

// Shadowing
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");
}

fn main() {

    var();

    shadowing();

}
