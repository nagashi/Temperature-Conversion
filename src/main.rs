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

fn temperature_conversion(temp_type: TemperatureUnit, num: i32) {
    match temp_type {
        TemperatureUnit::Fahrenheit => {
            let i = (num as f32 - 32_f32) * (5_f32 / 9_f32);
            match i.fract() {
                x if x != 0.0 => {
                    println!(
                        "\n({:?}°{} - 32) * (5/9) = {:.2}°{}",
                        num,
                        TemperatureUnit::Fahrenheit,
                        i,
                        TemperatureUnit::Celcius
                    );
                }
                _ => {
                    println!(
                        "\n({:?}°{} - 32) * (5/9) = {:?}°{}",
                        num,
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
                        "\n({:?}°{} * 9/5) + 32 = {:.2}°{}",
                        num,
                        TemperatureUnit::Celcius,
                        i,
                        TemperatureUnit::Fahrenheit
                    );
                }

                _ => {
                    println!(
                        "\n({:?}°{} - 32) * (5/9) = {:?}°{}",
                        num,
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

    'outer: loop {
        println!("\nEnter c to convert to Fahrenheit or f to convert to Celsius");

        let mut temp_scale = String::new(); // new() is an associated function of String type.

        io::stdin()
            .read_line(&mut temp_scale)
            .expect("Failed to read temperature type");

        let temp_scale = match temp_scale.trim().parse::<TemperatureUnit>() {
            Ok(c_f) => c_f,
            Err(_) => continue,
        };

        'inner: loop {
            match temp_scale {
                Celcius => println!("\nEnter an integer to convert Celsius to Fahrenheit."),
                Fahrenheit => println!("\nEnter an integer to convert Fahrenheit to Celsius."),
            }

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read amount of temperature");

            let temperature: i32 = match temperature.trim().parse::<i32>() {
                Ok(temp) => temp,
                Err(_) => continue 'inner,
            };

            temperature_conversion(temp_scale, temperature);
            break 'outer;
        }
    }
}
