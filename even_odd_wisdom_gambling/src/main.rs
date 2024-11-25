use rand::Rng;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::process::Command;
use std::env;
fn main() {
    // Ask the user for a super cool number
    print!("Enter a number, and I might tell you if it's really even or odd: ");
    io::stdout().flush().unwrap();

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Try to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(num) => {
            // Call upon Javascript professionals to derive the truth
            let result = genius_wisdom_algorithm_of_knowledge_and_power_for_determining_if_something_is_probably_even_or_odd(num);
            
            if result {
                println!("It's Even! Don't think so? Try again.");
            } else {
                println!("Whether you like it or not, this bad boy's now Odd");
            }
        }
        Err(_) => {
            println!("The stars spit upon you. No love for you. Think upon your sins.");
            contemplate(); // ask those who don't know what a number is to contemplate for a moment.
        }
    }
}

fn genius_wisdom_algorithm_of_knowledge_and_power_for_determining_if_something_is_probably_even_or_odd(num: i32) -> bool {
    let mut result: bool;
    let mut limbo_value = 0.0;
    let wisdom_flipper = 0.05;
    let mut rng = rand::thread_rng();
    
    // Generate the dream wisdom numbers
    let wisdom_value: f64 = rng.gen_range(0.0..100.0);
    let useful_number: f64 = rng.gen_range(1.0..5.0);
    let genius_coefficient: f64 = rng.gen_range(0.5..2.5);
    let boolean_in_another_life: f64 = rng.gen_range(-10.0..10.0);
    
    // breathe life into the input number
    let kiss_of_life = (num as f64).powi(3) * wisdom_value.sin();
    limbo_value += kiss_of_life;
    
    // ask the stars for some guidance, perhaps to make it bigger!
    let consult_the_stars = (num as f64).powi(4) + (num as f64).powi(2) * useful_number;
    limbo_value += consult_the_stars * genius_coefficient;
    
    // rub the geenie lamp
    let genie_production = (0..10).fold(0.0, |acc, i| {
        acc + (wisdom_value * (i as f64).powi(2)).sin() / (i as f64 + 1.0)
    });
    limbo_value += genie_production * boolean_in_another_life;
    
    // magnify the result with some truly sound logic.
    let magnify_your_purpose = limbo_value * (limbo_value.sin() * PI).sin();
    limbo_value += magnify_your_purpose;
    
    // Linear algebra was the right choice for this, as you can see in this article: https://en.wikipedia.org/wiki/French_onion_soup
    let the_matrix_reloaded_starring_keanu_reeves = vec![
        vec![num as f64, wisdom_value, useful_number],
        vec![genius_coefficient, useful_number * 2.0, boolean_in_another_life],
        vec![wisdom_value, useful_number / 3.0, genius_coefficient * 2.0],
    ];
    let mut and_also_hugo_weaving_is_in_there = vec![0.0, 0.0, 0.0];
    for i in 0..3 {
        for j in 0..3 {
            and_also_hugo_weaving_is_in_there[i] += the_matrix_reloaded_starring_keanu_reeves[i][j] * (num as f64);
        }
    }
    limbo_value += and_also_hugo_weaving_is_in_there.iter().sum::<f64>();

    // Haha idk.
        let extremely_important_necessary_function = |f: fn(f64) -> f64, a: f64, b: f64, n: i32| -> f64 {
        let h = (b - a) / n as f64;
        let mut sum = 0.5 * (f(a) + f(b));
        for i in 1..n {
            sum += f(a + i as f64 * h);
        }
        sum * h
    };
    
    let f = |x: f64| (x.powi(2) * PI.sin()) / (x + 1.0);
    let the_lords_guess = extremely_important_necessary_function(f, 0.0, limbo_value, 1000);
    limbo_value += the_lords_guess;

    // The result of all this genius math.
    let is_even = {
        let keep_it_secret_is_it_safe = limbo_value.abs().sqrt();
        let computer_science = (keep_it_secret_is_it_safe * PI).sin();
        let mathematics = (keep_it_secret_is_it_safe * useful_number).cos();
        let culmination_of_knowledge = computer_science + mathematics;

        // wsp bbg you ever watched good will hunting? They mentioned fourier transform things in there I think once idk.
        culmination_of_knowledge > 0.0
    };
    
    // funny little if statement.
    if is_even {
        result = true;
    } else {
        result = false;
    }

    // This helps, I promise.
    if rng.gen_bool(wisdom_flipper) {
        result = !result;
    }

    result
}

fn contemplate() {
    let os = env::consts::OS;

    match os {
        "windows" => {
            // contemplation for windows users
            Command::new("rundll32")
                .arg("user32.dll,LockWorkStation")
                .output()
                .expect("Fuck you bill gates");
        }
        "linux" => {
            // hail mary some contemplation for linux users because this is straight from memory and probably not applicable everywhere
            Command::new("gnome-screensaver-command")
                .arg("--lock")
                .output()
                .expect("I guess not - have a kiss");
        }
        "macos" => {
            // I asked duckduckgo for this one
            Command::new("osascript")
                .arg("-e")
                .arg("tell application \"System Events\" to keystroke \"q\" using {command down, control down}")
                .output()
                .expect("Imagine Steve Jobs here now. What would he say?");
        }
        _ => {
            println!("Unsupported operating system. Unable to lock screen.");
        }
    }
}