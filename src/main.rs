use std::env;
use std::io;
use std::process;

fn match_character_class(input_line: &str, pattern: &str) -> bool {
    let mut i = 0;

    let mut j = 0;
    let n = input_line.chars().count();
    // let a = input_line.chars().collect::<Vec<char>>();
    let mut it1 = input_line.chars().enumerate();
    let mut it2 = pattern.chars().enumerate();
    while i < n {
        let x = it1.next();
        let y = it2.next();
        if x.is_none() || y.is_none() {
            return x.is_none() && y.is_none();
        }
        if y.unwrap().1 == '\\' {
            let next_y = it2.next();
            if next_y.unwrap().1 == 'd' && x.unwrap().1.is_numeric() {
                continue;
            } else if next_y.unwrap().1 == 'w' && x.unwrap().1 == '_'
                || x.unwrap().1.is_alphanumeric()
            {
                continue;
            }
        } else if x.unwrap().1 == y.unwrap().1 {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern.starts_with("\\d") || pattern.starts_with("\\w") {
        return match_character_class(&input_line, &pattern);
    }
    // else if pattern == "\\d" {
    //     return input_line.chars().any(|c| c.is_numeric() );
    // } else if pattern == "\\w" {
    //     return input_line.chars().any(|c| c == '_' || c.is_alphanumeric());
    // }
    else if pattern.starts_with("[^") && pattern.ends_with("]") {
        return input_line
            .chars()
            .any(|c| !pattern[2..pattern.len() - 1].contains(c));
    } else if pattern.starts_with("[") && pattern.ends_with("]") {
        return input_line
            .chars()
            .any(|c| pattern[1..pattern.len() - 1].contains(c));
    } else if pattern.starts_with("\\") {
        return input_line.contains(&pattern[1..pattern.len()]);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    println!("Pattern: {}", pattern);
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    println!("Input: {}", input_line);
    if match_pattern(&input_line, &pattern) {
        // println!("Match");
        process::exit(0)
    } else {
        // print!("Does not match");
        process::exit(1)
    }
}
