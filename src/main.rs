fn main() {
    // Mutable vars
    let mut mutable = 5;
    println!("The value of x is: {}", mutable);
    mutable = 6;
    println!("The value of x is: {}", mutable);

    // Constants
    const ONE: u32 = 1;
    println!("The value of ONE is: {}", ONE);

    //Shadowing
    let shadow = 1;
    println!("Shadowed var is: {}", shadow);

    let shadow = "ffs";
    println!("Shadowed var is: {}", shadow);
}
