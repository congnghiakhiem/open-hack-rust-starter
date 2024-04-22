use crate::advanced::model::WeatherData;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::solutions;
use super::utils::{get_time, timeit};

#[allow(dead_code)]
fn main_template(
    f: fn(reader: BufReader<File>, station_data: &mut HashMap<String, WeatherData>) -> (),
) {
    use std::env;

    // Get the current working director
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the relative path to the file
    let relative_path = "src/advanced/small_data.txt";

    // Combine current directory with relative path
    let file_path = current_dir.join(relative_path);

    // Open the file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Initialize a HashMap to store weather station data
    let mut station_data: HashMap<String, WeatherData> = HashMap::new();

    f(reader, &mut station_data);

    // END CODE: Output the results
    let mut output = String::new();
    output.push('{');
    for (station, data) in station_data.iter() {
        let mean = data.calculate_mean();
        output.push_str(&format!(
            "{}={:.1}/{:.1}/{:.1}, ",
            station, data.min, mean, data.max
        ));
    }
    // Remove the trailing comma and space
    output.pop();
    output.pop();
    output.push('}');

    println!("{}", output);
}

#[allow(dead_code)]
fn main_cnk(
    f: fn(path: std::path::PathBuf) -> (),
) {
    use std::env;

    // Get the current working director
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Construct the relative path to the file
    let relative_path = "src/advanced/small_data.txt";

    // Combine current directory with relative path
    let file_path = current_dir.join(relative_path);

    f(file_path);

}

#[test]
fn benchmark_template() {
    timeit("template_code", || {
        main_template(solutions::template::template_solution)
    });
}

// #[test]
// fn benchmark_github_anhpham() {
//     timeit("github_anhpham", || {
//         main_template(solutions::github_anhpham::github_anhpham_solution)
//     });
// }

#[test]
fn benchmark_cnk() {
    timeit("cnk", || {
        main_cnk(solutions::github_congnghiakhiem::cnk_solution)
    });
}

#[test]
fn compare_time() -> Result<(), String> {
    let time_template_u64 = get_time("template", || {
        main_template(solutions::template::template_solution)
    });

    // let time_github_anhpham_u64 = get_time("github_anhpham", || {
    //     main_template(solutions::github_anhpham::github_anhpham_solution)
    // });

    let time_cnk_u64 = get_time("cnk", || {
        main_cnk(solutions::github_congnghiakhiem::cnk_solution)
    });

    // if time_template_u64 >= time_github_anhpham_u64 {
    //     Ok(())
    // } else {
    //     Err(format!("Template solution is not faster"))
    // }

    if time_template_u64 >= time_cnk_u64 {
        println!("{}", time_cnk_u64);
        Ok(())
    } else {
        Err(format!("Template solution is not faster"))
    }

}