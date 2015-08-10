use std::io;
use std::io::BufRead;

fn is_lowercase_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_letter(c: char) -> bool {
    is_lowercase_letter(c) || is_uppercase_letter(c)
}

fn rotate_lowercase(c: char, rotation: u32) -> char {
    std::char::from_u32(((c as u32 - 'a' as u32 + rotation) % ('z' as u32 - 'a' as u32 + 1)) + 'a' as u32).unwrap()
}

fn rotate_uppercase(c: char, rotation: u32) -> char {
    std::char::from_u32(((c as u32 - 'A' as u32 + rotation) % ('Z' as u32 - 'A' as u32 + 1)) + 'A' as u32).unwrap()
}

fn main() {
    let stdin = io::stdin();
    stdin.lock().lines().next();
    let information = stdin.lock().lines().next().unwrap().unwrap();
    let line_rotation = stdin.lock().lines().next().unwrap().unwrap();
    let rotation = line_rotation.parse::<u32>().unwrap();
    let encrypted : String = information.chars().map(|c| {
        if is_letter(c) {
            if is_lowercase_letter(c) {
                return rotate_lowercase(c, rotation);
            } else if is_uppercase_letter(c) {
                return rotate_uppercase(c, rotation);
            }
        }
        return c;
    }).collect();
    println!("{}", encrypted);
}
