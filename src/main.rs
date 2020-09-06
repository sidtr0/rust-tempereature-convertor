use std::io;

fn main() {

    println!("Choose 'f' or 'c' to start with. ");
    let mut choice_of_unit = String::new();

    io::stdin()
        .read_line(&mut choice_of_unit)
        .expect("Failed to read line. ");

    match choice_of_unit.trim() {
        "f" => {
            
            println!("Enter the tempereature in Fahrenheit. ");

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line. ");

            let temp = temp.trim().parse::<f32>().expect("Error encountered. ");
            println!("{}", temp);

            println!("{}", fahrenheit_to_celsius(temp));

        },
        
        "c" => {

            println!("Enter the tempereature in Celsius. ");

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line. ");

            let mut temp = temp.trim().parse::<f32>().expect("Error encountered. ");
            println!("{}", temp);
            
            println!("{}", celsius_to_fahrenheit(temp));
            
        },

        _ => println!("incorrect choice. "),
    }

}

// C / 5 = (F - 32) / 9
// C = (F - 32) * 5 / 9
// F = ((C / 5)* 9) + 32

fn celsius_to_fahrenheit(t: f32) -> f32 {
    ((t / 5.00) * 9.00) + 32.00
}

fn fahrenheit_to_celsius(t: f32) -> f32 {
    (t - 32.00) * 5.00 / 9.00
}