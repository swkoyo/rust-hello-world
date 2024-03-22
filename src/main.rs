use core::panic;
use std::io;

fn main() {
    let mut temp_type = String::new();
    let mut temp = String::new();

    println!("Are you entering fahrenheit (f) or celcius (c)?");
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");

    let temp_type = temp_type.trim();

    if temp_type != "c" && temp_type != "f" {
        panic!("Invalid type given");
    }

    println!("Enter the temperature value: ");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => {
            if temp_type == "c" {
                convert_c_to_f(num)
            } else {
                convert_f_to_c(num)
            }
        }
        Err(_) => panic!("Enter a valid temperature"),
    };

    println!("Converted {temp_type}: {temp}");
}

fn convert_f_to_c(f_temp: f64) -> f64 {
    (f_temp - 32.0) * (5.0 / 9.0)
}

fn convert_c_to_f(c_temp: f64) -> f64 {
    (c_temp * (9.0 / 5.0)) + 32.0
}
