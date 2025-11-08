fn main() {

    let mut input = String::new();
    println!("Enter age:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: u32 = input.trim().parse().expect("Please enter a valid number");
    if age < 18 {
        println!("You are a minor.");
    } else if age > 120 {
        println!("Please enter a valid age.");
    } else {
        println!("You are an adult.");
    }



}
