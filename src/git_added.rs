use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn get_added(content: &str) -> String {
    let added_lines: Vec<&str> = Vec::new();
    let thread_shared_lines = Arc::new(Mutex::new(added_lines));
    let lines: Vec<&str> = content.split("\n").collect();
    lines.par_iter().for_each(|line| {
        if line.starts_with("+") {
            let temp_lines = thread_shared_lines.lock();
            match temp_lines {
                Ok(mut temp_lines) => {
                    temp_lines.push(line);
                }
                Err(_) => {
                    println!("Cound Not Add Line {:#?}", line);
                }
            }
        }
    });
    match thread_shared_lines.lock() {
        Ok(thread_shared_lines) => {
            return thread_shared_lines.join("\n");
        }
        Err(_) => {
            println!("Could Not Read Lines");
        }
    }
    return "".to_string();
}
