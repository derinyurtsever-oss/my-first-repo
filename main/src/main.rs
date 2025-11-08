fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(), // empty string
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}


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
    
    let mut name = String::new();
    println!("Enter your name:");
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    let name = capitalize_first(name);
    println!("Nice to meet you, {}!", name);
    println!("How are you today, {}?", name);
    let mut mood = String::new();
    std::io::stdin().read_line(&mut mood).expect("Failed to read line");
    let mood = mood.trim();
    match mood {
        "good" | "great" | "fantastic" => println!("I'm glad to hear that, {}!", name),
        "bad" | "not good" | "terrible" => println!("I'm sorry to hear that, {}. I hope things get better soon.", name),
        _ => println!("Thanks for sharing, {}.", name),
    }

    println!("How is the weather today, {}?", name);
    let mut weather = String::new();
    std::io::stdin().read_line(&mut weather).expect("Failed to read line");
    let weather = weather.trim();
    match weather {
        "sunny" | "clear" => println!("It's a beautiful day, {}!", name),
        "rainy" | "cloudy" | "stormy" => println!("Stay safe in this weather, {}.", name),
        _ => println!("Thanks for the update, {}.", name),
    }
    println!("Where are you located, {}?", name);
    let mut location = String::new();
    std::io::stdin().read_line(&mut location).expect("Failed to read line");
    let location = location.trim();
    println!("{} sounds like a nice place!", location);

    struct Person {
        name: String,
        age: u32,
        mood: String,
        weather: String,
        location: String,
    }
    let person = Person {
        name,
        age,
        mood: mood.to_string(),
        weather: weather.to_string(),
        location: location.to_string()
    };
    impl std::fmt::Debug for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Person {{ name: {}, age: {}, mood: {}, weather in {}: {} }}", self.name, self.age, self.mood, self.location, self.weather)
        }
    }    println!("Person details: {:?}", person);


}
