//! # more_cargo
//!
//! `more_cargo` is a collection of examples and dummy code to
//!  illustrate the points of this session
//! 
use std::{
    thread::sleep,
    time::Duration,
};
use std::io::Write; 
use rand::Rng;
use termion::clear;

/// Return a circle whose color depends on the percentage
/// Below just an example of inserting an image in the docs
/// <img src="https://bluss.github.io/ndarray/images/split_at.svg" width="300px" height="271px">
/// # Examples
///
/// ```
/// let percentage = 5;
/// let answer = more_cargo::paint_point(arg);
///
/// assert_eq!('\u{1F534}', answer);
/// ```
/// # Panics
///
/// ```
/// let percentage = 101;
/// let answer = more_cargo::paint_point(arg);
///
/// assert_eq!('\u{1F534}', answer);
/// ```
/// # Errors
/// # Safety
fn paint_point(loading_percentage: i32) -> char{
    let red_emoji = '\u{1F534}';
    let yellow_emoji = '\u{1F7E1}';
    let green_emoji = '\u{1F7E2}';

    if loading_percentage <30 {
        red_emoji
    } else if 30<=loading_percentage && loading_percentage<=70 {
        yellow_emoji
    } else if 70<loading_percentage && loading_percentage<=100  {
        green_emoji
    } else{
        panic!("Not possible percentage")
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    for i in 1..100 {
        print!("\r {}Loading {}%...", paint_point(i), i);
        std::io::stdout().flush().unwrap();

        let waiting_miliseconds = rng.gen_range(0..1000);
        sleep(Duration::from_millis(waiting_miliseconds));
    }
    println!("{}", clear::CurrentLine);
    println!("Loaded!");
}
