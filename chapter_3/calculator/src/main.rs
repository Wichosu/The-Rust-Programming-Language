use std::io;

fn main() {
    let error_io_message = String::from("Failed to read line");

    println!("Fahrenheit Celcius Calculator");
    println!("Select and option");

    loop {
        println!("1.-Fahrenheit to Celcius");
        println!("2.-Celcius to Fahrenheit");
        println!("3.-Exit");

        let mut option = String::new();
        
        io::stdin()
        .read_line(&mut option)
        .expect(&error_io_message);

        let option: i32 = option.trim().parse().expect("Select a valid option");
        
        if option == 1 {
            println!("Fahrenheit to Celcius");
            println!("Please type the temperature in Fahrenheit");

            let mut temperature = String::new();
            
            io::stdin()
            .read_line(&mut temperature)
            .expect(&error_io_message);
        
            let temperature: f32 = temperature.trim().parse().expect("Not a number");
        
            print!("{temperature} Fahrenheit equals ");

            let temperature = ((temperature - 32.0) * 5.0) / 9.0;
            
            println!("{temperature} Celcius");
        } else if option == 2 {
            println!("Celcius to Fahrenheit");
            println!("Please type the temperature in Celcius");
            
            let mut temperature = String::new();
            
            io::stdin()
            .read_line(&mut temperature)
            .expect(&error_io_message);
        
            let temperature: f32 = temperature.trim().parse().expect("Not a number");
        
            print!("{temperature} Celcius equals ");
        
            let temperature = (temperature * (9.0 / 5.0)) + 32.0;
        
            println!("{temperature} Farenheit");
        } else if option == 3 {
            println!("Goodbye!");
            break
        }
    }
}
