use std::{fmt, io, str::FromStr};

enum TemperatureUnit {
    Fahrenheit,
    Celcius,
}

impl FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "c" | "celcius" => Ok(TemperatureUnit::Celcius),
            "f" | "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
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

fn temperature_conversion(temp_type: TemperatureUnit, num: f64) {
    match temp_type {
        TemperatureUnit::Fahrenheit => {
            let i = (num as f64 - 32_f64) * (5_f64 / 9_f64);
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
                        i as u64,
                        TemperatureUnit::Celcius
                    );
                }
            }
        }
        TemperatureUnit::Celcius => {
            let i = (num as f64 * (9_f64 / 5.0)) + 32_f64;
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
                        i as u64,
                        TemperatureUnit::Fahrenheit
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    CF,
    C,
    F,
}

fn process(msg: Message) {
    let quit = |msg| println!("\nType \"quit\" to end the program or\n{msg}");

    match msg {
        Message::CF => {
            quit("Enter C to convert to Fahrenheit or F to convert to Celsius");
        }
        Message::C => {
            quit("Enter a number to convert Celsius to Fahrenheit.");
        }
        Message::F => {
            quit("Enter a number to convert Fahrenheit to Celsius.");
        }
    }
}

fn main() {
    use TemperatureUnit::*;

    'outer: loop {
        let mut temp_scale = String::new();

        process(Message::CF);

        io::stdin()
            .read_line(&mut temp_scale)
            .expect("Failed to reade line");

        if temp_scale.trim().to_string().to_lowercase() == "quit" {
            break 'outer;
        }

        let temp_scale = match temp_scale.trim().parse::<TemperatureUnit>() {
            Ok(c_f) => c_f,
            Err(_) => continue 'outer,
        };

        'inner: loop {
            match temp_scale {
                Celcius => process(Message::C),
                Fahrenheit => process(Message::F),
            }

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read amount of temperature");

            if temperature.trim().to_string().to_lowercase() == "quit" {
                break 'outer;
            }

            let temperature: f64 = match temperature.trim().parse::<f64>() {
                Ok(temp) => temp,
                Err(_) => continue 'inner,
            };

            temperature_conversion(temp_scale, temperature);
            continue 'outer;
        }
    }
}