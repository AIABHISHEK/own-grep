use std::env;
use std::io;
use std::process;

fn match_character_class(input_line: &str, pattern: &str) -> bool {
    let mut i = 0;

    // let mut j = 0;
    let n = input_line.chars().count();
    let a = input_line.chars().collect::<Vec<char>>();
    let b = pattern.chars().collect::<Vec<char>>();

    // for i in 0..n {
    //     //print chars
    //     print!("{}", a[i]);
    //     if i < n - 1 {
    //         print!(", ");
    //     }
    // }
    // print!("\n");
    let m = pattern.chars().count();
    // for i in 0..b.len() {
    //     //print chars
    //     if b[i] == '\\' {
    //         println!("5666");
    //     }
    //     print!("{}", b[i]);
    //     if i < b.len() - 1 {
    //         print!(", ");
    //     }
    // }

    while i < n {
        let mut j = 0;
        let mut k = 0;
        while j < m {
            if b[j] == '\\' {
                j += 1;
                continue;
            }
            if j < m
                && i + k  < n
                && (a[i + k] == b[j]
                    || (j > 0
                        && ((b[j] == 'w'
                            && b[j - 1] == '\\'
                            && (a[i + k] == '_' || a[i + k].is_alphanumeric()))
                            || (b[j] == 'd' && b[j - 1] == '\\' && a[i + k].is_numeric()))))
            {
                println!("{},{}", b[j], a[i + k]);
                j += 1;
                k += 1;
            } else {
                break;
            }
            if j == m {
                return true;
            }
        }

        i += 1;
    }
    return false;
}

fn match_start_line(input_line: &str, pattern: &str) -> bool {
    if input_line.len() >= pattern.len() - 1 {
        return &input_line[0..pattern.len() - 1] == &pattern[1..pattern.len()];
    }
    return false;
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
    else if pattern.starts_with("^") {
        return match_start_line(input_line, pattern);
    }
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
        println!("Match");
        process::exit(0)
    } else {
        print!("Does not match");
        process::exit(1)
    }
}
