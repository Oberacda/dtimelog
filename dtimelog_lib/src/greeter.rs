// Copyright 2015 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chrono::offset::{Local, TimeZone};
use chrono::{DateTime, Duration};
use plotters::prelude::*;

/// Object that displays a greeting.
#[repr(C)]
pub struct Greeter {
    greeting: String,
}

/// Implementation of Greeter.
impl Greeter {
    /// Constructs a new `Greeter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtimelog_lib::greeter::Greeter;
    ///
    /// let greeter = Greeter::new("Hello");
    /// ```
    pub fn new(greeting: &str) -> Greeter {
        Greeter {
            greeting: greeting.to_string(),
        }
    }

    /// Returns the greeting as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtimelog_lib::greeter::Greeter;
    ///
    /// let greeter = Greeter::new("Hello");
    /// let greeting = greeter.greeting("World");
    /// ```
    pub fn greeting(&self, thing: &str) -> String {
        format!("{} {}", &self.greeting, thing)
    }

    /// Prints the greeting.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtimelog_lib::greeter::Greeter;
    ///
    /// let greeter = Greeter::new("Hello");
    /// greeter.greet("World");
    /// ```
    pub fn greet(&self, thing: &str) {
        let connection = sqlite::open("dtimelog.db").unwrap();

        let query = "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
        ";
        connection.execute(query).unwrap();
        println!("{} {}", &self.greeting, thing);
    }
}

fn parse_time(t: &str) -> DateTime<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
}

fn get_data() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![
        ("2019-04-25", 130.06, 131.37, 128.83, 129.15),
        ("2019-04-24", 125.79, 125.85, 124.52, 125.01),
        ("2019-04-23", 124.10, 125.58, 123.83, 125.44),
        ("2019-04-22", 122.62, 124.00, 122.57, 123.76),
        ("2019-04-18", 122.19, 123.52, 121.30, 123.37),
        ("2019-04-17", 121.24, 121.85, 120.54, 121.77),
        ("2019-04-16", 121.64, 121.65, 120.10, 120.77),
        ("2019-04-15", 120.94, 121.58, 120.57, 121.05),
        ("2019-04-12", 120.64, 120.98, 120.37, 120.95),
        ("2019-04-11", 120.54, 120.85, 119.92, 120.33),
        ("2019-04-10", 119.76, 120.35, 119.54, 120.19),
        ("2019-04-09", 118.63, 119.54, 118.58, 119.28),
        ("2019-04-08", 119.81, 120.02, 118.64, 119.93),
        ("2019-04-05", 119.39, 120.23, 119.37, 119.89),
        ("2019-04-04", 120.10, 120.23, 118.38, 119.36),
        ("2019-04-03", 119.86, 120.43, 119.15, 119.97),
        ("2019-04-02", 119.06, 119.48, 118.52, 119.19),
        ("2019-04-01", 118.95, 119.10, 118.10, 119.02),
        ("2019-03-29", 118.07, 118.32, 116.96, 117.94),
        ("2019-03-28", 117.44, 117.58, 116.13, 116.93),
        ("2019-03-27", 117.87, 118.21, 115.52, 116.77),
        ("2019-03-26", 118.62, 118.70, 116.85, 117.91),
        ("2019-03-25", 116.56, 118.01, 116.32, 117.66),
        ("2019-03-22", 119.50, 119.59, 117.04, 117.05),
        ("2019-03-21", 117.13, 120.82, 117.09, 120.22),
        ("2019-03-20", 117.39, 118.75, 116.71, 117.52),
        ("2019-03-19", 118.09, 118.44, 116.99, 117.65),
        ("2019-03-18", 116.17, 117.61, 116.05, 117.57),
        ("2019-03-15", 115.34, 117.25, 114.59, 115.91),
        ("2019-03-14", 114.54, 115.20, 114.33, 114.59),
    ]
}

pub fn plot() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data();
    let root = SVGBackend::new("stock.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (to_date, from_date) = (
        parse_time(data[0].0) + Duration::days(1),
        parse_time(data[29].0) - Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;

    chart.draw_series(
        data.iter().map(|x| {
            CandleStick::new(parse_time(x.0), x.1, x.2, x.3, x.4, GREEN.filled(), RED, 15)
        }),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Greeter;

    #[test]
    fn test_greeting() {
        let hello = Greeter::new("Hi");
        assert_eq!("Hi Rust", hello.greeting("Rust"));
    }
}
