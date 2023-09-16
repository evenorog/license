use license::License;
use std::io;

fn main() {
    let stdin = io::stdin();
    for id in stdin.lines() {
        if let Ok(license) = id.unwrap().parse::<&dyn License>() {
            println!("{}", license.text());
        }
    }
}
