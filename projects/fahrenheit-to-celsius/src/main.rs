use std::io;

fn main() {
    println!("Enter the temperature");
    
    let mut temperature : String = String::new();

    io::stdin().read_line(&mut temperature).expect("Failed to read temperature");
    
    let mut temperature = temperature.trim().to_string();
    let unit : Option<char> = temperature.pop();
    let temperature : f64 = temperature.parse().expect("Failed to parse temperature as integer");

    if unit == Some('F') {
        let converted = (temperature - 32.0) * 5.0/9.0;
        println!("Conversion to Celsius: {converted}");
    }
    else if unit == Some('C') {
        let converted = (temperature * 5.0/9.0) + 32.0;
        println!("Conversion to Fahrenheit: {converted}");
    }
    else {
        println!("Invalid temperature format.");
    }
}
