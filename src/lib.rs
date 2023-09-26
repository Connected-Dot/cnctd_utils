use std::{path::Path, thread, time::Duration, io::{Write, stdout}};

use colored::Colorize;
use crossterm::{execute, terminal::{Clear, ClearType}};
use figlet_rs::FIGfont;

pub fn get_relative_path(parent: &Path, child: &Path) -> Option<String> {
    child.strip_prefix(parent).ok().map(|path| path.to_str().unwrap().to_string())
}

pub fn get_exe_dir() -> String {
    std::env::current_exe().unwrap().to_str().unwrap().to_string()
}

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn display_logo(word: &str, animate: bool) {
    let standard_font = FIGfont::standard().unwrap();
    let mut partial_word = String::new();

    for ch in word.chars() {
        partial_word.push(ch);
        let figure = standard_font.convert(&partial_word).unwrap();
        let logo = figure.to_string().cyan().to_string();

        clear_terminal();
        println!("{}", logo);

        if animate {
            thread::sleep(Duration::from_millis(100));
        }
    }
}

pub fn get_logo(word: &str) -> String {
    let standard_font = FIGfont::standard().unwrap();
    standard_font.convert(word).unwrap().to_string().cyan().to_string()
}

pub fn print_separator(length: u8, animate: bool) {
    println!("");
    for _i in 1..length {
        print!("-");
        std::io::stdout().flush().unwrap();
        if animate { thread::sleep(Duration::from_millis(10)) }
    }
    println!("");
}