use std::io;

fn main() {
    loop {
        println!("Choose option:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");

        println!("Enter your option: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u8 = match input.trim().parse() {
            Ok(res) => res,
            Err(_e) => {
                println!("Invalid option.");
                continue;
            }
        };

        match input {
            1 => {
                println!("Enter the celsius value:");
                let input = read_f32_input("Invalid celsius value.");

                println!("Fahrenheit equivalent is: {}", cel_to_fah(input))
            }
            2 => {
                println!("Enter the fahrenheit value:");
                let input = read_f32_input("Invalid fahrenheit value.");

                println!("Celsius equivalent is: {}", fah_to_cel(input))
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option")
            }
        }
    }
}

fn read_f32_input(error_message: &str) -> f32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect(&error_message)
}

fn cel_to_fah(number: f32) -> f32 {
    (number * 1.8) + 32.0
}

fn fah_to_cel(number: f32) -> f32 {
    ((number - 32.0) * 5.0) / 9.0
}
