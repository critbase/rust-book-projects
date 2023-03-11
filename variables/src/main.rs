const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("x is: {x}");
    // shadowing != mut, as this x is a completely new variable that uses the same "name"
    let x = x + 1;
    println!("x is: {x}");

    {
        // new scope created with curly brackets
        let x = x * 2;
        println!("inside scope, x is: {x}");
    }

    println!("outside scope, x is: {x}");

    println!("three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}
