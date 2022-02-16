use std::{fmt, io, str::FromStr};

enum TemperatureUnit {
    Fahrenheit,
    Celcius,
}

impl FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "C" => Ok(TemperatureUnit::Celcius),
            "f" | "F" => Ok(TemperatureUnit::Fahrenheit),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureUnit::Fahrenheit => write!(f, "F"),
            TemperatureUnit::Celcius => write!(f, "C"),
        }
    }
}

fn temperature_conversion(temp_type: TemperatureUnit, num: f32) {
    match temp_type {
        TemperatureUnit::Fahrenheit => {
            let i = (num as f32 - 32_f32) * (5_f32 / 9_f32);
            match i.fract() {
                x if x != 0.0 => {
                    println!(
                        "\n({num}°{} - 32) * (5/9) = {:.1}°{}",
                        TemperatureUnit::Fahrenheit,
                        i,
                        TemperatureUnit::Celcius
                    );
                }
                _ => {
                    println!(
                        "\n({num}°{} - 32) * (5/9) = {:?}°{}",
                        TemperatureUnit::Fahrenheit,
                        i as i32,
                        TemperatureUnit::Celcius
                    );
                }
            }
        }

        TemperatureUnit::Celcius => {
            let i = (num as f32 * (9_f32 / 5.0)) + 32_f32;
            match i.fract() {
                x if x != 0.0 => {
                    println!(
                        "\n({num}°{} * 9/5) + 32 = {:.1}°{}",
                        TemperatureUnit::Celcius,
                        i,
                        TemperatureUnit::Fahrenheit
                    );
                }

                _ => {
                    println!(
                        "\n({num}°{} - 32) * (5/9) = {:?}°{}",
                        TemperatureUnit::Celcius,
                        i as i32,
                        TemperatureUnit::Fahrenheit
                    );
                }
            }
        }
    }
}

fn main() {
    use TemperatureUnit::*;

    let quit = |param| println!("\nType \"quit\" to end the program or\n{}", param);
    let mut vec = Vec::new();
    vec.push("Enter c to convert to Fahrenheit or f to convert to Celsius");
    vec.push("Enter a number to convert Celsius to Fahrenheit.");
    vec.push("Enter a number to convert Fahrenheit to Celsius.");

    'outer: loop {
        let mut temp_scale = String::new();

        quit(vec[0]);

        io::stdin()
            .read_line(&mut temp_scale)
            .expect("Failed to reade line");

        if temp_scale.trim().to_string().to_lowercase() == "quit" {
            break 'outer;
        }

        let temp_scale = match temp_scale.trim().parse::<TemperatureUnit>() {
            Ok(c_f) => c_f,
            Err(_) => continue,
        };

        'inner: loop {
            match temp_scale {
                Celcius => quit(vec[1]),
                Fahrenheit => quit(vec[2]),
            }

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read amount of temperature");

            if temperature.trim().to_string().to_lowercase() == "quit" {
                break 'outer;
            }

            let temperature: f32 = match temperature.trim().parse::<f32>() {
                Ok(temp) => temp,
                Err(_) => continue 'inner,
            };

            temperature_conversion(temp_scale, temperature);
            continue 'outer;
        }
    }
}
