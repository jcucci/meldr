// Conflict playground - test file for weavr development
use std::io::Write;

fn greet(name: &str) -> String {
    format!("Welcome, {}!", name)
}

fn calculate(a: i32, b: i32) -> i32 {
    a * b
}

fn get_version() -> &'static str {
    "2.0.0-beta"
}
