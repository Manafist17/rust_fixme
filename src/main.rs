use std::fmt::{self, Display, Formatter};

/* Demonstrates printing of a user defined struct using println! macro.*/

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}
/**
This code is a modified version of the format code used for Displaying the city information the
basic premis is that we are grabing the data from the buffer and putting into the desiered format in our case
the desired format is
red: (Red input from buffer),green: (Green input from buffer),blue: (blue input from buffer)
**/
impl Display for Color {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(
            f,
            "{}: {},{}: {:.3},{}: {:.3}",
            "red", self.red, "green", self.green, "blue", self.blue
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {
            name: "Glassboro",
            lat: 39.702892,
            lon: -75.111839,
        },
        City {
            name: "Mullica Hill",
            lat: 39.73928,
            lon: -75.224072,
        },
        City {
            name: "Swedesboro",
            lat: 39.747616,
            lon: -75.310463,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Hint : Fix the code so you can print it using {}
        println!("{}", *color);
    }
}
